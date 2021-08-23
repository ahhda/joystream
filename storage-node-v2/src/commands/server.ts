import { flags } from '@oclif/command'
import { createApp } from '../services/webApi/app'
import ApiCommandBase from '../command-base/ApiCommandBase'
import logger, { initElasticLogger } from '../services/logger'

import { performSync } from '../services/sync/synchronizer'
import sleep from 'sleep-promise'
import _ from 'lodash'

/**
 * CLI command:
 * Starts the storage node server.
 *
 * @remarks
 * Shell command: "server"
 */
export default class Server extends ApiCommandBase {
  static description = 'Starts the storage node server.'

  static flags = {
    worker: flags.integer({
      char: 'w',
      required: true,
      description: 'Storage provider worker ID',
    }),
    uploads: flags.string({
      char: 'd',
      required: true,
      description: 'Data uploading directory (absolute path).',
    }),
    port: flags.integer({
      char: 'o',
      required: true,
      description: 'Server port.',
    }),
    sync: flags.boolean({
      char: 's',
      description: 'Enable data synchronization.',
      default: false, // TODO: turn on by default
    }),
    syncInterval: flags.integer({
      char: 'i',
      description: 'Interval between syncronizations (in minutes)',
      default: 1,
    }),
    queryNodeHost: flags.string({
      char: 'q',
      required: true,
      description: 'Query node host and port (e.g.: some.com:8081)',
    }),
    syncWorkersNumber: flags.integer({
      char: 'r',
      required: false,
      description: 'Sync workers number (max async operations in progress).',
      default: 20,
    }),
    elasticSearchHost: flags.string({
      char: 'e',
      required: false,
      description: 'Elasticsearch host and port (e.g.: some.com:8081).',
    }),
    ...ApiCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(Server)

    let elasticUrl
    if (!_.isEmpty(flags.elasticSearchHost)) {
      elasticUrl = `http://${flags.elasticSearchHost}`
      initElasticLogger(elasticUrl)
    }

    const queryNodeUrl = `http://${flags.queryNodeHost}/graphql`
    logger.info(`Query node endpoint set: ${queryNodeUrl}`)


    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    if (flags.sync) {
      logger.info(`Synchronization enabled.`)

      runSyncWithInterval(
        flags.worker,
        queryNodeUrl,
        flags.uploads,
        flags.syncWorkersNumber,
        flags.syncInterval
      )
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    try {
      const port = flags.port
      const workerId = flags.worker ?? 0
      const maxFileSize = await api.consts.storage.maxDataObjectSize.toNumber()
      logger.debug(`Max file size runtime parameter: ${maxFileSize}`)

      const app = await createApp(
        api,
        account,
        workerId,
        flags.uploads,
		maxFileSize,
        this.config,
        queryNodeUrl,
        elasticUrl,
      )
      logger.info(`Listening on http://localhost:${port}`)
      app.listen(port)
    } catch (err) {
      logger.error(`Error: ${err}`)
    }
  }

  // Override exiting.
  /* eslint-disable @typescript-eslint/no-empty-function */
  async finally(): Promise<void> {}
}

function runSyncWithInterval(
  workerId: number,
  queryNodeUrl: string,
  uploadsDirectory: string,
  syncWorkersNumber: number,
  syncIntervalMinutes: number
) {
  setTimeout(async () => {
    // TODO: restore
    //   const sleepIntevalInSeconds = syncIntervalMinutes * 60 * 1000
    const sleepIntevalInSeconds = 10000
    logger.info(`Sync paused for ${syncIntervalMinutes} minute(s).`)
    await sleep(sleepIntevalInSeconds)
    logger.info(`Resume syncing....`)

    try {
      await performSync(
        workerId,
        syncWorkersNumber,
        queryNodeUrl,
        uploadsDirectory
      )
    } catch (err) {
      logger.error(`Critical sync error: ${err}`)
    }

    runSyncWithInterval(
      workerId,
      queryNodeUrl,
      uploadsDirectory,
      syncWorkersNumber,
      syncIntervalMinutes
    )
  }, 0)
}
