import { Vec, Option, Tuple, BTreeSet, UInt } from '@polkadot/types'
import { bool, u64, u32, Null, Bytes } from '@polkadot/types/primitive'
import { JoyStructDecorated, JoyEnum, ChannelId, MemberId, Balance, Hash, BlockNumber, BalanceOf } from '../common'

import { GenericAccountId as AccountId } from '@polkadot/types/generic/AccountId'
import { DataObjectId, DataObjectCreationParameters } from '../storage'

export class CuratorId extends u64 {}
export class CuratorGroupId extends u64 {}
export class ChannelCategoryId extends u64 {}
export class VideoId extends u64 {}
export class VideoCategoryId extends u64 {}
export class PlaylistId extends u64 {}
export class PersonId extends u64 {}
export class SeriesId extends u64 {}
export class ChannelOwnershipTransferRequestId extends u64 {}
export class MaxNumber extends u32 {}
export class IsCensored extends bool {}
export class VideoPostId extends u64 {}
export class ReactionId extends u64 {}
export class CurrencyOf extends BalanceOf {}
export class CurrencyAmount extends CurrencyOf {}

// NFT types

export class Royalty extends UInt {}
export class IsExtended extends bool {}

export class EnglishAuctionDetails extends JoyStructDecorated({
  extension_period: BlockNumber,
  auction_duration: BlockNumber,
}) {}

export class OpenAuctionDetails extends JoyStructDecorated({
  bid_lock_duration: BlockNumber,
}) {}

export class AuctionType extends JoyEnum({
  English: EnglishAuctionDetails,
  Open: OpenAuctionDetails,
}) {}

export class Bid extends JoyStructDecorated({
  bidder: MemberId,
  bidder_account_id: AccountId,
  amount: Balance,
  made_at_block: BlockNumber,
}) {}

export class Auction extends JoyStructDecorated({
  starting_price: Balance,
  buy_now_price: Option.with(Balance),
  auction_type: AuctionType,
  minimal_bid_step: Balance,
  last_bid: Option.with(Bid),
  starts_at: BlockNumber,
  whitelist: BTreeSet.with(MemberId),
}) {}

export class TransactionalStatus extends JoyEnum({
  Idle: Null,
  InitiatedOfferToMember: Tuple.with([MemberId, Option.with(Balance)]),
  Auction,
  BuyNow: Balance,
}) {}

export class NFTOwner extends JoyEnum({
  ChannelOwner: Null,
  Member: MemberId,
}) {}

export class OwnedNFT extends JoyStructDecorated({
  owner: NFTOwner,
  transactional_status: TransactionalStatus,
  creator_royalty: Option.with(Royalty),
}) {}

export class AuctionParams extends JoyStructDecorated({
  auction_type: AuctionType,
  starting_price: Balance,
  minimal_bid_step: Balance,
  buy_now_price: Option.with(Balance),
  starts_at: Option.with(BlockNumber),
  whitelist: BTreeSet.with(MemberId),
}) {}

// end of NFT types

export class StorageAssets extends JoyStructDecorated({
  object_creation_list: Vec.with(DataObjectCreationParameters),
  expected_data_size_fee: Balance,
}) {}

export class CuratorGroup extends JoyStructDecorated({
  curators: BTreeSet.with(CuratorId),
  active: bool,
}) {}

export class ContentActor extends JoyEnum({
  Curator: Tuple.with([CuratorGroupId, CuratorId]),
  Member: MemberId,
  Lead: Null,
}) {}

export class ChannelOwner extends JoyEnum({
  Member: MemberId,
  Curators: CuratorGroupId,
}) {}

export class Channel extends JoyStructDecorated({
  owner: ChannelOwner,
  num_videos: u64,
  is_censored: bool,
  reward_account: Option.with(AccountId),
  collaborators: BTreeSet.with(MemberId),
  moderators: BTreeSet.with(MemberId),
  cumulative_payout_earned: Balance,
}) {}

export class ChannelCreationParameters extends JoyStructDecorated({
  assets: Option.with(StorageAssets),
  meta: Option.with(Bytes),
  reward_account: Option.with(AccountId),
  collaborators: BTreeSet.with(MemberId),
  moderators: BTreeSet.with(MemberId),
}) {}

export class ChannelUpdateParameters extends JoyStructDecorated({
  assets_to_upload: Option.with(StorageAssets),
  new_meta: Option.with(Bytes),
  reward_account: Option.with(Option.with(AccountId)),
  assets_to_remove: BTreeSet.with(DataObjectId),
  collaborators: Option.with(BTreeSet.with(MemberId)),
}) {}

export class ChannelOwnershipTransferRequest extends JoyStructDecorated({
  channel_id: ChannelId,
  new_owner: ChannelOwner,
  payment: Balance,
  new_reward_account: Option.with(AccountId),
}) {}

export class ChannelCategory extends JoyStructDecorated({
  // No runtime information is currently stored for a Category.
}) {}

export class ChannelCategoryCreationParameters extends JoyStructDecorated({
  meta: Bytes,
}) {}

export class ChannelCategoryUpdateParameters extends JoyStructDecorated({
  new_meta: Bytes,
}) {}

export class VideoCategory extends JoyStructDecorated({
  // No runtime information is currently stored for a Category.
}) {}

export class VideoCategoryCreationParameters extends JoyStructDecorated({
  meta: Bytes,
}) {}

export class VideoCategoryUpdateParameters extends JoyStructDecorated({
  new_meta: Bytes,
}) {}

export class Video extends JoyStructDecorated({
  in_channel: ChannelId,
  in_series: Option.with(SeriesId),
  is_censored: bool,
  enable_comments: bool,
  video_post_id: Option.with(VideoPostId),
  nft_status: Option.with(OwnedNFT),
}) {}

export class VideoCreationParameters extends JoyStructDecorated({
  assets: Option.with(StorageAssets),
  meta: Option.with(Bytes),
  enable_comments: bool,
}) {}

export class VideoUpdateParameters extends JoyStructDecorated({
  assets_to_upload: Option.with(StorageAssets),
  new_meta: Option.with(Bytes),
  assets_to_remove: BTreeSet.with(DataObjectId),
  enable_comments: Option.with(bool),
}) {}

export class Playlist extends JoyStructDecorated({
  in_channel: ChannelId,
}) {}

export class PlaylistCreationParameters extends JoyStructDecorated({
  meta: Bytes,
}) {}

export class PlaylistUpdateParameters extends JoyStructDecorated({
  new_meta: Bytes,
}) {}

export class EpisodeParemters extends JoyEnum({
  NewVideo: VideoCreationParameters,
  ExistingVideo: VideoId,
}) {}

export class Season extends JoyStructDecorated({
  episodes: Vec.with(VideoId),
}) {}

export class SeasonParameters extends JoyStructDecorated({
  assets: Option.with(StorageAssets),
  episodes: Option.with(Vec.with(Option.with(EpisodeParemters))),
  meta: Option.with(Bytes),
}) {}

export class Series extends JoyStructDecorated({
  in_channel: ChannelId,
  seasons: Vec.with(Season),
}) {}

export class SeriesParameters extends JoyStructDecorated({
  assets: Option.with(StorageAssets),
  seasons: Option.with(Vec.with(Option.with(SeasonParameters))),
  meta: Option.with(Bytes),
}) {}

export class PersonController extends JoyEnum({
  Member: MemberId,
  Curators: Null,
}) {}

export class Person extends JoyStructDecorated({
  controlled_by: PersonController,
}) {}

export class PersonCreationParameters extends JoyStructDecorated({
  assets: StorageAssets,
  meta: Bytes,
}) {}

export class PersonUpdateParameters extends JoyStructDecorated({
  assets: Option.with(StorageAssets),
  meta: Option.with(Bytes),
}) {}

export class PersonActor extends JoyEnum({
  Member: MemberId,
  Curator: CuratorId,
}) {}

export class VideoMigrationConfig extends JoyStructDecorated({
  current_id: VideoId,
  final_id: VideoId,
}) {}
export class ChannelMigrationConfig extends JoyStructDecorated({
  current_id: ChannelId,
  final_id: ChannelId,
}) {}

export class VideoPostType extends JoyEnum({
  Description: Null,
  Comment: VideoPostId,
}) {}

export class VideoPost extends JoyStructDecorated({
  author: ContentActor,
  bloat_bond: Balance,
  replies_count: VideoPostId,
  post_type: VideoPostType,
  video_reference: VideoId,
}) {}

export class Side extends JoyEnum({
  Left: Null,
  Right: Null,
}) {}

export class ProofElement extends JoyStructDecorated({
  hash: Hash,
  side: Side,
}) {}

export class VideoPostCreationParameters extends JoyStructDecorated({
  post_type: VideoPostType,
  video_reference: VideoId,
}) {}

export class VideoPostDeletionParameters extends JoyStructDecorated({
  witness: Option.with(Hash),
  rationale: Option.with(Bytes),
}) {}

export class PullPayment extends JoyStructDecorated({
  channel_id: ChannelId,
  cumulative_payout_claimed: Balance,
  reason: Hash,
}) {}

export class ModeratorSet extends BTreeSet.with(MemberId) {}

export const contentTypes = {
  CuratorId,
  CuratorGroupId,
  CuratorGroup,
  ContentActor,
  StorageAssets,
  Channel,
  ChannelOwner,
  ChannelCategoryId,
  ChannelCategory,
  ChannelCategoryCreationParameters,
  ChannelCategoryUpdateParameters,
  ChannelCreationParameters,
  ChannelUpdateParameters,
  ChannelOwnershipTransferRequestId,
  ChannelOwnershipTransferRequest,
  Video,
  VideoId,
  VideoCategoryId,
  VideoCategory,
  VideoCategoryCreationParameters,
  VideoCategoryUpdateParameters,
  VideoCreationParameters,
  VideoUpdateParameters,
  Person,
  PersonId,
  PersonController,
  PersonActor,
  PersonCreationParameters,
  PersonUpdateParameters,
  Playlist,
  PlaylistId,
  PlaylistCreationParameters,
  PlaylistUpdateParameters,
  SeriesId,
  Series,
  Season,
  SeriesParameters,
  SeasonParameters,
  EpisodeParemters,
  MaxNumber,
  IsCensored,
  VideoMigrationConfig,
  ChannelMigrationConfig,
  // Added in Olympia:
  VideoPostId,
  ReactionId,
  VideoPostType,
  VideoPost,
  Side,
  ProofElement,
  VideoPostCreationParameters,
  VideoPostDeletionParameters,
  PullPayment,
  ModeratorSet,
  // NFT
  Royalty,
  IsExtended,
  EnglishAuctionDetails,
  OpenAuctionDetails,
  AuctionType,
  Bid,
  Auction,
  TransactionalStatus,
  NFTOwner,
  OwnedNFT,
  AuctionParams,
  CurrencyOf,
  CurrencyAmount,
}

export default contentTypes
