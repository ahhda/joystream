import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { apiModuleByGroup } from '../../Api'
import chalk from 'chalk'
import { createType } from '@joystream/types'

export default class WorkingGroupsFillOpening extends WorkingGroupsCommandBase {
  static description = "Allows filling working group opening that's currently in review. Requires lead access."
  static args = [
    {
      name: 'wgOpeningId',
      required: true,
      description: 'Working Group Opening ID',
    },
  ]

  static flags = {
    ...WorkingGroupsCommandBase.flags,
  }

  async run(): Promise<void> {
    const { args } = this.parse(WorkingGroupsFillOpening)

    // Lead-only gate
    const lead = await this.getRequiredLeadContext()

    const openingId = parseInt(args.wgOpeningId)
    const opening = await this.getOpeningForLeadAction(openingId)

    const applicationIds = await this.promptForApplicationsToAccept(opening)

    await this.sendAndFollowNamedTx(
      await this.getDecodedPair(lead.roleAccount),
      apiModuleByGroup[this.group],
      'fillOpening',
      [openingId, createType('BTreeSet<ApplicationId>', applicationIds)]
    )

    this.log(chalk.green(`Opening ${chalk.magentaBright(openingId.toString())} successfully filled!`))
    this.log(
      chalk.green('Accepted working group application IDs: ') +
        chalk.magentaBright(applicationIds.length ? applicationIds.join(chalk.green(', ')) : 'NONE')
    )
  }
}
