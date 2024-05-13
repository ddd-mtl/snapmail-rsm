/* This file is generated by zits. Do not edit manually */

import {ZomeName, FunctionName} from '@holochain/client';


/** Array of all zome function names in "delivery" */
export const deliveryFunctionNames: FunctionName[] = [
	"entry_defs", 
	"get_zome_info", 
	"get_dna_info",





	"commit_pending_item",
	"commit_private_chunks",
	"commit_private_manifest",
	"complete_manifest",
	"determine_missing_chunks",
	"distribute_parcel",
	"get_all_private_manifests",
	"get_all_local_public_manifests",
	"get_delivery_state",
	"get_distribution_state",
	"get_manifest",
	"get_chunk",
	"get_notice",
	"get_parcel_notice",
	"get_notice_state",
	"notify_public_parcel",
	"publish_chunks",
	"publish_chunk",
	"publish_manifest",
	"pull_inbox",
	"pull_public_parcels",
	"pull_public_parcels_details",
	"get_parcel_ref",
	"query_all_Distribution",
	"query_all_DeliveryNotice",
	"query_DeliveryNotice",
	"query_all_NoticeAck",
	"query_NoticeAck",
	"query_all_NoticeReply",
	"query_all_ReplyAck",
	"query_all_ReceptionProof",
	"query_ReceptionProof",
	"query_all_ReceptionAck",
	"query_all_private_manifests",
	"query_all_public_manifests",
	"query_all_public_chunks",
	"query_all_private_chunks",
	"remove_public_parcel",
	"respond_to_notice",
	"scan_incomplete_manifests",
	"scan_orphan_chunks",
	"fetch_chunk",];


/** Generate tuple array of function names with given zomeName */
export function generateDeliveryZomeFunctionsArray(zomeName: ZomeName): [ZomeName, FunctionName][] {
   const fns: [ZomeName, FunctionName][] = [];
   for (const fn of deliveryFunctionNames) {
      fns.push([zomeName, fn]);
   }
   return fns;
}


/** Tuple array of all zome function names with default zome name "delivery" */
export const deliveryZomeFunctions: [ZomeName, FunctionName][] = generateDeliveryZomeFunctionsArray("delivery");
