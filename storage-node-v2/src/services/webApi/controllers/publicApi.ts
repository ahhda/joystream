import * as express from 'express'
import { acceptPendingDataObjects } from '../../runtime/extrinsics'
import {
  UploadTokenRequest,
  UploadTokenBody,
  createUploadToken,
  verifyTokenSignature,
} from '../../helpers/auth'
import { hashFile } from '../../../services/helpers/hashing'
import { KeyringPair } from '@polkadot/keyring/types'
import { ApiPromise } from '@polkadot/api'
import { parseBagId } from '../../../services/helpers/bagIdParser'
import fs from 'fs'
import { Membership } from '@joystream/types/members'
const fsPromises = fs.promises

interface UploadRequest {
  dataObjectId: number
  storageBucketId: number
  bagId: string
}

export async function upload(
  req: express.Request,
  res: express.Response
): Promise<void> {
  const uploadRequest: UploadRequest = req.body

  try {
    const fileObj = getFileObject(req)

    const hash = await hashFile(fileObj.path)
    const newPath = fileObj.path.replace(fileObj.filename, hash)

    // Overwrites existing file.
    await fsPromises.rename(fileObj.path, newPath)

    const api = getApi(res)
    const bagId = parseBagId(api, uploadRequest.bagId)
    await acceptPendingDataObjects(
      api,
      bagId,
      getAccount(res),
      getWorkerId(res),
      uploadRequest.storageBucketId,
      [uploadRequest.dataObjectId]
    )
    res.status(201).json({
      file: 'received',
    })
  } catch (err) {
    res.status(410).json({
      type: 'upload',
      message: err.toString(),
    })
  }
}

export async function authToken(
  req: express.Request,
  res: express.Response
): Promise<void> {
  try {
    const account = getAccount(res)
    const tokenRequest = getTokenRequest(req)
    const api = getApi(res)

    await validateTokenRequest(api, tokenRequest)

    const tokenBody: UploadTokenBody = {
      timestamp: Date.now(),
      ...tokenRequest.data,
    }
    const signedToken = createUploadToken(tokenBody, account)

    res.status(201).json({
      token: signedToken,
    })
  } catch (err) {
    res.status(410).json({
      type: 'authtoken',
      message: err.toString(),
    })
  }
}

function getFileObject(req: express.Request): Express.Multer.File {
  if (req.file) {
    return req.file
  }

  const files = req.files as Express.Multer.File[]
  if (files && files.length > 0) {
    return files[0]
  }

  throw new Error('No file uploaded')
}

function getWorkerId(res: express.Response): number {
  if (res.locals.workerId || res.locals.workerId === 0) {
    return res.locals.workerId
  }

  throw new Error('No Joystream worker ID loaded.')
}

function getAccount(res: express.Response): KeyringPair {
  if (res.locals.storageProviderAccount) {
    return res.locals.storageProviderAccount
  }

  throw new Error('No Joystream account loaded.')
}

function getApi(res: express.Response): ApiPromise {
  if (res.locals.api) {
    return res.locals.api
  }

  throw new Error('No Joystream API loaded.')
}

function getTokenRequest(req: express.Request): UploadTokenRequest {
  const tokenRequest = req.body as UploadTokenRequest
  if (tokenRequest) {
    return tokenRequest
  }

  throw new Error('No token request provided.')
}

async function validateTokenRequest(
  api: ApiPromise,
  tokenRequest: UploadTokenRequest
): Promise<void> {
  const result = verifyTokenSignature(tokenRequest, tokenRequest.data.accountId)

  if (!result) {
    throw new Error('Invalid upload token request signature.')
  }

  const membership = (await api.query.members.membershipById(
    tokenRequest.data.memberId
  )) as Membership

  if (
    membership.controller_account.toString() !== tokenRequest.data.accountId
  ) {
    throw new Error(`Provided controller account and member id don't match.`)
  }
}
