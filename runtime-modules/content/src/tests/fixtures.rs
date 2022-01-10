use super::curators;
use super::mock::*;
use crate::*;
use frame_support::assert_ok;
use frame_support::traits::Currency;
use sp_runtime::DispatchError;

// type aliases
type AccountId = <Test as frame_system::Trait>::AccountId;
type VideoId = <Test as Trait>::VideoId;

// fixtures
pub struct CreatePostFixture {
    sender: AccountId,
    actor: ContentActor<CuratorGroupId, CuratorId, MemberId>,
    params: PostCreationParameters<Test>,
}

impl CreatePostFixture {
    pub fn default() -> Self {
        Self {
            sender: DEFAULT_MEMBER_ACCOUNT_ID,
            actor: ContentActor::Member(DEFAULT_MEMBER_ID),
            params: PostCreationParameters::<Test> {
                post_type: PostType::<Test>::VideoPost,
                video_reference: VideoId::one(),
            },
        }
    }

    pub fn with_sender(self, sender: AccountId) -> Self {
        Self { sender, ..self }
    }

    pub fn with_actor(self, actor: AccountId) -> Self {
        Self { actor, ..self }
    }

    pub fn with_params(self, params: PostCreationParameters<Test>) -> Self {
        Self { params, ..self }
    }

    pub fn call_and_assert(&self, expected_result: DispatchResult) {
        let origin = Origin::signed(self.sender.clone());
        let balance_pre = Balances::usable_balance(self.sender);
        let initial_bloat_bond = Content::compute_initial_bloat_bond();
        let post_id = Content::next_post_id();
        let replies_count_pre = match &self.params.post_type {
            PostType::<Test>::Comment(parent_id) => {
                Content::ensure_post_exists(&self.params.video_reference, parent_id)
                    .unwrap_or(Zero::zero())
            }
            PostType::<Test>::VideoPost => Zero::zero(),
        };
        let video_pre = Content::video_by_id(&self.params.video_reference);

        let actual_result = Content::create_post(origin, self.actor.clone(), self.params.clone());

        let balance_post = Balances::usable_balance(self.sender);
        let replies_count_post = match &self.params.post_type {
            PostType::<Test>::Comment(parent_id) => {
                Content::ensure_post_exists(&self.params.video_reference, parent_id)
                    .unwrap_or(Zero::zero())
            }
            PostType::<Test>::VideoPost => Zero::zero(),
        };
        let video_post = Content::video_by_id(&self.params.video_reference);

        assert_eq!(actual_result, expected_result);

        match actual_result {
            Ok(()) => {
                assert_eq!(balance_pre.saturating_sub(balance_post), initial_bloat_bond);
                assert_eq!(post_id.saturating_add(One::one()), Content::next_post_id());
                match &self.params.post_type {
                    PostType::<Test>::VideoPost => {
                        assert_eq!(Some(post_id), video_post.video_post_id);
                    }
                    PostType::<Test>::Comment(parent_id) => {
                        assert_eq!(replies_post, replies_pre.saturating_add(One::one()));
                    }
                }
            }
            Err(_) => {
                assert_eq!(balance_pre, balance_post);
                assert_eq!(post_id, Content::next_post_id());
                match &self.params.post_type {
                    PostType::<Test>::VideoPost => {
                        assert_eq!(video_pre, video_post);
                    }
                    PostType::<Test>::Comment(parent_id) => {
                        assert_eq!(replies_post, replies_pre);
                    }
                }
            }
        }
    }
}

// helpers
pub fn create_default_member_channel_with_video() {
    assert_ok!(Content::create_channel(
        Origin::signed(DEFAULT_MEMBER_ACCOUNT_ID),
        ContentActor::Member(DEFAULT_MEMBER_ID),
        ChannelCreationParameters::<Test> {
            assets: vec![],
            meta: vec![],
            reward_account: None,
            moderator_set: None,
        }
    ));

    assert_ok!(Content::create_video(
        Origin::signed(DEFAULT_MEMBER_ACCOUNT_ID),
        ContentActor::Member(DEFAULT_MEMBER_ID),
        ChannelId::one(),
        VideoCreationParameters {
            assets: vec![],
            meta: vec![],
            enable_comments: true,
        }
    ));
}
