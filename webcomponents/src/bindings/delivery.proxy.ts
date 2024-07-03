/* This file is generated by zits. Do not edit manually */

import {DELIVERY_INTERGRITY_ZOME_NAME, DELIVERY_ZOME_NAME, DIRECTORY_PATH, DIRECT_SEND_CHUNK_TIMEOUT_MS, DIRECT_SEND_TIMEOUT_MS, REMOTE_ENDPOINT, DeliveryEntry, DeliveryNoticeQueryField, DeliveryProtocol, DeliveryState, DeliveryTipProtocol, DistributionState, DistributionStrategy, ItemKind, NoticeAckQueryField, NoticeState, ParcelKind, ReceptionProofQueryField, BroadcastInput, CommitParcelInput, CommitPendingItemInput, DeliveryMessage, DeliveryNotice, DeliveryProperties, DeliverySummary, DistributeParcelInput, Distribution, FetchChunkInput, FullDistributionState, GetDeliveryStateInput, GetNoticeOutput, NoticeAck, NoticeReply, ParcelChunk, ParcelDescription, ParcelManifest, ParcelReference, PendingItem, PublicParcelRecord, ReceptionAck, ReceptionProof, ReplyAck, RespondToNoticeInput, } from './delivery.types';
import {
WebsocketConnectionOptions,
/** types.ts */
HoloHash,
AgentPubKey,
DnaHash,
WasmHash,
EntryHash,
ActionHash,
AnyDhtHash,
ExternalHash,
KitsuneAgent,
KitsuneSpace,
HoloHashB64,
AgentPubKeyB64,
DnaHashB64,
WasmHashB64,
EntryHashB64,
ActionHashB64,
AnyDhtHashB64,
InstalledAppId,
Signature,
CellId,
DnaProperties,
RoleName,
InstalledCell,
Timestamp,
Duration,
HoloHashed,
NetworkInfo,
FetchPoolInfo,
/** hdk/action.ts */
SignedActionHashed,
RegisterAgentActivity,
ActionHashed,
ActionType,
Action,
NewEntryAction,
Dna,
AgentValidationPkg,
InitZomesComplete,
CreateLink,
DeleteLink,
OpenChain,
CloseChain,
Update,
Delete,
Create,
/** hdk/capabilities.ts */
CapSecret,
CapClaim,
GrantedFunctionsType,
GrantedFunctions,
ZomeCallCapGrant,
CapAccessType,
CapAccess,
CapGrant,
///** hdk/countersigning.ts */
//CounterSigningSessionData,
//PreflightRequest,
//CounterSigningSessionTimes,
//ActionBase,
//CounterSigningAgents,
//PreflightBytes,
//Role,
//CountersigningAgentState,
/** hdk/dht-ops.ts */
DhtOpType,
DhtOp,
getDhtOpType,
getDhtOpAction,
getDhtOpEntry,
getDhtOpSignature,
/** hdk/entry.ts */
EntryVisibility,
AppEntryDef,
EntryType,
EntryContent,
Entry,
/** hdk/record.ts */
Record as HcRecord,
RecordEntry as HcRecordEntry,
/** hdk/link.ts */
AnyLinkableHash,
ZomeIndex,
LinkType,
LinkTag,
RateWeight,
RateBucketId,
RateUnits,
Link,
/** api/admin/types.ts */
InstalledAppInfoStatus,
DeactivationReason,
DisabledAppReason,
StemCell,
ProvisionedCell,
ClonedCell,
CellType,
CellInfo,
AppInfo,
MembraneProof,
FunctionName,
ZomeName,
ZomeDefinition,
IntegrityZome,
CoordinatorZome,
DnaDefinition,
ResourceBytes,
ResourceMap,
CellProvisioningStrategy,
CellProvisioning,
DnaVersionSpec,
DnaVersionFlexible,
AppRoleDnaManifest,
AppRoleManifest,
AppManifest,
AppBundle,
AppBundleSource,
NetworkSeed,
ZomeLocation,
   } from '@holochain/client';

import {
/** Common */
DhtOpHashB64,
//DnaHashB64, (duplicate)
//AnyDhtHashB64, (duplicate)
DhtOpHash,
/** DnaFile */
DnaFile,
DnaDef,
Zomes,
WasmCode,
/** entry-details */
EntryDetails,
RecordDetails,
Details,
DetailsType,
EntryDhtStatus,
/** Validation */
ValidationStatus,
ValidationReceipt,
   } from '@holochain-open-dev/core-types';

/** User defined external dependencies */
import {EntryDefIndex} from './deps.types';

import {ZomeProxy} from '@ddd-qc/lit-happ';
import {deliveryFunctionNames} from './delivery.fn';
import {DeliveryUnitEnum, DeliveryLinkType} from './delivery.integrity';

/**
 *
 */
export class DeliveryProxy extends ZomeProxy {
  static readonly DEFAULT_ZOME_NAME = "zDelivery";
  static readonly FN_NAMES = deliveryFunctionNames;
  static readonly ENTRY_TYPES = Object.values(DeliveryUnitEnum);
  static readonly LINK_TYPES = Object.values(DeliveryLinkType);
 
  async commitPrivateChunks(chunks: ParcelChunk[]): Promise<EntryHash[]> {
    return this.callBlocking('commit_private_chunks', chunks);
  }

  async commitPrivateManifest(manifestArg: ParcelManifest): Promise<EntryHash> {
    return this.call('commit_private_manifest', manifestArg);
  }

  async completeManifest(manifestEh: EntryHash): Promise<[EntryHash, EntryHash | EntryHash[]][] | null> {
    return this.call('complete_manifest', manifestEh);
  }

  async determineMissingChunks(manifestEh: EntryHash): Promise<EntryHash[]> {
    return this.call('determine_missing_chunks', manifestEh);
  }

  async distributeParcel(input: DistributeParcelInput): Promise<ActionHash> {
    return this.callBlocking('distribute_parcel', input);
  }

  async fetchPublicManifest(manifestEh: EntryHash): Promise<[ParcelManifest, Timestamp, AgentPubKey]> {
    return this.call('fetch_public_manifest', manifestEh);
  }

  async fetchChunk(chunkEh: EntryHash): Promise<ParcelChunk> {
    return this.call('fetch_chunk', chunkEh);
  }

  async getDeliveryState(input: GetDeliveryStateInput): Promise<DeliveryState> {
    return this.call('get_delivery_state', input);
  }

  async getDistributionState(distributionAh: ActionHash): Promise<FullDistributionState> {
    return this.call('get_distribution_state', distributionAh);
  }

  async getNotice(distributionAh: ActionHash): Promise<GetNoticeOutput | null> {
    return this.call('get_notice', distributionAh);
  }

  async getNoticeState(noticeEh: EntryHash): Promise<[NoticeState, EntryHash[]]> {
    return this.call('get_notice_state', noticeEh);
  }

  async processInbox(): Promise<ActionHash[]> {
    return this.call('process_inbox', null);
  }

  async publishChunks(chunks: ParcelChunk[]): Promise<EntryHash[]> {
    return this.call('publish_chunks', chunks);
  }

  async publishChunk(chunk: ParcelChunk): Promise<EntryHash> {
    return this.call('publish_chunk', chunk);
  }

  async publishPendingItem(input: CommitPendingItemInput): Promise<ActionHash> {
    return this.callBlocking('publish_pending_item', input);
  }

  async publishPublicParcel(manifestArg: ParcelManifest): Promise<EntryHash> {
    return this.callZomeBlockPostCommit('PublicParcel','publish_public_parcel', manifestArg);
  }

  async pullPublicParcelsDetails(): Promise<void> {
    return this.call('pull_public_parcels_details', null);
  }

  async fetchParcelRef(prEh: EntryHash): Promise<ParcelReference | null> {
    return this.call('fetch_parcel_ref', prEh);
  }

  async queryDeliveryNotice(queryField: DeliveryNoticeQueryField): Promise<[DeliveryNotice, Timestamp][]> {
    return this.call('query_DeliveryNotice', queryField);
  }

  async queryNoticeAck(field: NoticeAckQueryField): Promise<NoticeAck[]> {
    return this.call('query_NoticeAck', field);
  }

  async queryReceptionProof(field: ReceptionProofQueryField): Promise<[EntryHash, Timestamp, ReceptionProof] | null> {
    return this.call('query_ReceptionProof', field);
  }

  async queryAll(): Promise<void> {
    return this.call('query_all', null);
  }

  async queryAllDistribution(): Promise<void> {
    return this.call('query_all_Distribution', null);
  }

  async queryAllDeliveryNotice(): Promise<void> {
    return this.call('query_all_DeliveryNotice', null);
  }

  async queryAllNoticeAck(): Promise<void> {
    return this.call('query_all_NoticeAck', null);
  }

  async queryAllNoticeReply(): Promise<void> {
    return this.call('query_all_NoticeReply', null);
  }

  async queryAllReplyAck(): Promise<void> {
    return this.call('query_all_ReplyAck', null);
  }

  async queryAllReceptionProof(): Promise<void> {
    return this.call('query_all_ReceptionProof', null);
  }

  async queryAllReceptionAck(): Promise<void> {
    return this.call('query_all_ReceptionAck', null);
  }

  async queryAllPrivateManifests(): Promise<void> {
    return this.call('query_all_private_manifests', null);
  }

  async queryAllPublicManifests(): Promise<void> {
    return this.call('query_all_public_manifests', null);
  }

  async queryAllPublicChunks(): Promise<void> {
    return this.call('query_all_public_chunks', null);
  }

  async queryAllPrivateChunks(): Promise<void> {
    return this.call('query_all_private_chunks', null);
  }

  async respondToNotice(input: RespondToNoticeInput): Promise<EntryHash> {
    return this.call('respond_to_notice', input);
  }

  async scanIncompleteManifests(): Promise<EntryHash[]> {
    return this.call('scan_incomplete_manifests', null);
  }

  async scanOrphanChunks(): Promise<[EntryHash[], EntryHash[]]> {
    return this.call('scan_orphan_chunks', null);
  }

  async unpublishPublicParcel(ppEh: EntryHash): Promise<ActionHash> {
    return this.callBlocking('unpublish_public_parcel', ppEh);
  }

  async pullChunk(input: FetchChunkInput): Promise<[ParcelChunk, Link | null] | null> {
    return this.call('pull_chunk', input);
  }
}
