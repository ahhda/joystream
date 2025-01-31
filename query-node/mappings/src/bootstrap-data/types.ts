export type MemberJson = {
  memberId: string
  rootAccount: string
  controllerAccount: string
  handle: string
  about?: string
  avatarUri?: string
  registeredAtTime: number
  registeredAtBlock: number
}

export type StorageSystemJson = {
  id: string
  blacklist: string[]
  storageBucketsPerBagLimit: number
  distributionBucketsPerBagLimit: number
  uploadingBlocked: boolean
  dataObjectFeePerMb: number | string
  storageBucketMaxObjectsCountLimit: number | string
  storageBucketMaxObjectsSizeLimit: number | string
}

export type WorkingGroupJson = {
  name: string
  budget: number
}

export type VideoCategoryJson = {
  id: string
  name: string
  createdInBlock: number
  createdAt: string
  updatedAt: string
}

export type ChannelCategoryJson = {
  id: string
  name: string
  createdInBlock: number
  createdAt: string
  updatedAt: string
}

export type MembershipSystemJson = {
  defaultInviteCount: number
  membershipPrice: number
  referralCut: number
  invitedInitialBalance: number
}
