use crate::parser::{HttpMethod, Request, RequestMap, RequestSet, RequestType, ResponseType};
use graph_core::resource::ResourceIdentity;
use inflector::Inflector;
use std::collections::HashMap;

#[allow(deprecated)]
pub fn get_custom_requests(
    resource_identity: ResourceIdentity,
) -> Option<HashMap<String, RequestSet>> {
    let vec = {
        match resource_identity {
            ResourceIdentity::Drive | ResourceIdentity::Drives => vec![
                Request {
                    path: "/{{drive_item_id}}/children".to_string(),
                    method: HttpMethod::GET,
                    method_name: "list_children".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Collection,
                    tag: Default::default(),
                    operation_id: "drives.ListChildren".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/activities".to_string(),
                    method: HttpMethod::GET,
                    method_name: "get_item_activities".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Collection,
                    tag: Default::default(),
                    operation_id: "drives.GetActivities".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}".to_string(),
                    method: HttpMethod::PATCH,
                    method_name: "update_items".to_string(),
                    param_size: 1,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.UpdateItems".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}".to_string(),
                    method: HttpMethod::DELETE,
                    method_name: "delete_items".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.DeleteItems".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}".to_string(),
                    method: HttpMethod::GET,
                    method_name: "get_items".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.GetItems".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/children".to_string(),
                    method: HttpMethod::POST,
                    method_name: "create_folder".to_string(),
                    param_size: 1,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.CreateFolder".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_root}}/root/children".to_string(),
                    method: HttpMethod::POST,
                    method_name: "create_root_folder".to_string(),
                    param_size: 0,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.CreateRootFolder".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/copy".to_string(),
                    method: HttpMethod::POST,
                    method_name: "copy_item".to_string(),
                    param_size: 1,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.CopyItem".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/versions".to_string(),
                    method: HttpMethod::GET,
                    method_name: "list_item_versions".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Collection,
                    tag: Default::default(),
                    operation_id: "drives.ListItemVersions".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/versions/{{id2}}/restoreVersion".to_string(),
                    method: HttpMethod::POST,
                    method_name: "restore_item_versions".to_string(),
                    param_size: 2,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.RestoreItemVersions".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item}}/thumbnails".to_string(),
                    method: HttpMethod::GET,
                    method_name: "list_thumbnails".to_string(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.ListThumbnails".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}".to_string(),
                    method: HttpMethod::GET,
                    method_name: "get_thumbnail".to_string(),
                    param_size: 3,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.GetThumbnail".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}/content".to_string(),
                    method: HttpMethod::GET,
                    method_name: "get_thumbnail_binary".to_string(),
                    param_size: 3,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.GetThumbnailBinary".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                // TODO: Setting files as the body in a request macro
                Request {
                    path: "/{{drive_item_id}}/content".to_string(),
                    method: HttpMethod::PUT,
                    method_name: "upload_replace".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Upload,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.UploadReplace".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/content".to_string(),
                    method: HttpMethod::GET,
                    method_name: "get_item_content".to_string(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.GetItemContent".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}".to_string(),
                    method: HttpMethod::POST,
                    method_name: "move_items".to_string(),
                    param_size: 1,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::SerdeJson,
                    tag: Default::default(),
                    operation_id: "drives.MoveItems".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_root}}/root/children".to_string(),
                    method: HttpMethod::GET,
                    method_name: "list_root_children".to_string(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Collection,
                    tag: Default::default(),
                    operation_id: "drives.ListRootChildren".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "{{drive_root}}/activities".to_string(),
                    method: HttpMethod::GET,
                    method_name: "list_root_activities".to_string(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Collection,
                    tag: Default::default(),
                    operation_id: "drives.ListRootActivities".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "{{drive_root}}/root/delta".to_string(),
                    method: HttpMethod::GET,
                    method_name: "delta".to_string(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::Delta,
                    tag: Default::default(),
                    operation_id: "drives.Delta".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
                Request {
                    path: "/{{drive_item_id}}/checkin".to_string(),
                    method: HttpMethod::POST,
                    method_name: "check_in_item".to_string(),
                    param_size: 1,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: false,
                    response: ResponseType::NoContent,
                    tag: Default::default(),
                    operation_id: "drives.CheckIn".to_string(),
                    operation_mapping: "drives".to_string(),
                    doc: None,
                },
            ],
            ResourceIdentity::Items => vec![Request {
                path: "/items/{{RID}}".into(),
                method: HttpMethod::DELETE,
                method_name: "delete_items".into(),
                param_size: 1,
                has_body: false,
                request_type: RequestType::Normal,
                has_rid: true,
                response: ResponseType::NoContent,
                tag: "drives.driveItem".into(),
                operation_id: "items.DeleteItems".into(),
                operation_mapping: "items".into(),
                doc: Some("# Delete navigation property items".into()),
            }],
            ResourceIdentity::Pages => vec![
                Request {
                    path: "/pages/{{RID}}".into(),
                    method: HttpMethod::DELETE,
                    method_name: "delete_pages".into(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "pages".into(),
                    operation_id: "me.onenote.pages.DeletePages".into(),
                    operation_mapping: "me.onenote.pages".into(),
                    doc: None,
                },
                Request {
                    path: "/pages/{{RID}}/content".into(),
                    method: HttpMethod::PATCH,
                    method_name: "update_page_content".into(),
                    param_size: 0,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::SerdeJson,
                    tag: "pages".into(),
                    operation_id: "me.onenote.pages.UpdatePageContent".into(),
                    operation_mapping: "me.onenote.pages".into(),
                    doc: None,
                },
                Request {
                    path: "/pages/{{RID}}/content".into(),
                    method: HttpMethod::GET,
                    method_name: "download_page".into(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Download,
                    has_rid: true,
                    response: ResponseType::Download,
                    tag: "pages".into(),
                    operation_id: "me.onenote.pages.DownloadPage".into(),
                    operation_mapping: "me.onenote.pages".into(),
                    doc: None,
                },
                Request {
                    path: "/pages/{{RID}}/content".into(),
                    method: HttpMethod::GET,
                    method_name: "download_page".into(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::AsyncDownload,
                    has_rid: true,
                    response: ResponseType::AsyncDownload,
                    tag: "pages".into(),
                    operation_id: "me.onenote.pages.DownloadPage".into(),
                    operation_mapping: "me.onenote.pages".into(),
                    doc: None,
                },
            ],
            ResourceIdentity::Groups => vec![
                Request {
                    path: "/groups/{{RID}}/owners/{{id}}/$ref".into(),
                    method: HttpMethod::DELETE,
                    method_name: "remove_owner".into(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "groups".into(),
                    operation_id: "groups.RemoveOwner".into(),
                    operation_mapping: "groups".into(),
                    doc: None,
                },
                Request {
                    path: "/groups/{{RID}}/members/{{id}}/$ref".into(),
                    method: HttpMethod::DELETE,
                    method_name: "remove_member".into(),
                    param_size: 1,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "groups".into(),
                    operation_id: "groups.RemoveMember".into(),
                    operation_mapping: "groups".into(),
                    doc: None,
                },
                Request {
                    path: "/groups/{{RID}}/members/$ref".into(),
                    method: HttpMethod::POST,
                    method_name: "add_member".into(),
                    param_size: 0,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "groups".into(),
                    operation_id: "groups.AddMember".into(),
                    operation_mapping: "groups".into(),
                    doc: None,
                },
                Request {
                    path: "/groups/{{RID}}/owners/$ref".into(),
                    method: HttpMethod::POST,
                    method_name: "add_owner".into(),
                    param_size: 0,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "groups".into(),
                    operation_id: "groups.AddOwner".into(),
                    operation_mapping: "groups".into(),
                    doc: None,
                },
            ],
            ResourceIdentity::Conversations => vec![Request {
                path: "/conversations/{{RID}}".into(),
                method: HttpMethod::DELETE,
                method_name: "delete_conversations".into(),
                param_size: 0,
                has_body: false,
                request_type: RequestType::Normal,
                has_rid: true,
                response: ResponseType::NoContent,
                tag: "conversations".into(),
                operation_id: "groups.conversations.DeleteConversations".into(),
                operation_mapping: "groups.conversations".into(),
                doc: None,
            }],
            ResourceIdentity::Messages => vec![
                Request {
                    path: "/messages/{{RID}}/move".into(),
                    method: HttpMethod::POST,
                    method_name: "move_message".into(),
                    param_size: 0,
                    has_body: true,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::SerdeJson,
                    tag: "messages".into(),
                    operation_id: "messages.MoveMessage".into(),
                    operation_mapping: "messages".into(),
                    doc: None,
                },
                Request {
                    path: "/messages/{{RID}}".into(),
                    method: HttpMethod::DELETE,
                    method_name: "delete_messages".into(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "messages".into(),
                    operation_id: "messages.DeleteMessages".into(),
                    operation_mapping: "messages".into(),
                    doc: None,
                },
                Request {
                    path: "/messages/{{RID}}/$value".into(),
                    method: HttpMethod::GET,
                    method_name: "get_message_content".into(),
                    param_size: 0,
                    has_body: false,
                    request_type: RequestType::Normal,
                    has_rid: true,
                    response: ResponseType::NoContent,
                    tag: "messages".into(),
                    operation_id: "messages.GetMessageContent".into(),
                    operation_mapping: "messages".into(),
                    doc: None,
                },
            ],
            ResourceIdentity::ChildFolders => vec![Request {
                path: "/childFolders/{{RID}}/move".into(),
                method: HttpMethod::POST,
                method_name: "move_child_folders".into(),
                param_size: 0,
                has_body: true,
                request_type: RequestType::Normal,
                has_rid: true,
                response: ResponseType::SerdeJson,
                tag: "childFolders".into(),
                operation_id: "childFolders.MoveChildFolders".into(),
                operation_mapping: "childFolders".into(),
                doc: None,
            }],
            ResourceIdentity::Attachments => vec![Request {
                path: "/attachments/{{RID}}/$value".into(),
                method: HttpMethod::GET,
                method_name: "get_content".into(),
                param_size: 0,
                has_body: false,
                request_type: RequestType::Normal,
                has_rid: true,
                response: ResponseType::NoContent,
                tag: "attachments".into(),
                operation_id: "attachments.GetContent".into(),
                operation_mapping: "attachments".into(),
                doc: None,
            }],
            ResourceIdentity::Contacts => vec![Request {
                path: "/contacts/{{RID}}".into(),
                method: HttpMethod::DELETE,
                method_name: "delete_contacts".into(),
                param_size: 0,
                has_body: false,
                request_type: RequestType::Normal,
                has_rid: true,
                response: ResponseType::NoContent,
                tag: "contacts".into(),
                operation_id: "contacts.DeleteContacts".into(),
                operation_mapping: "contacts".into(),
                doc: None,
            }],
            _ => vec![],
        }
    };

    if vec.is_empty() {
        None
    } else {
        let mut request_set = RequestSet::default();

        for request in vec {
            let mut request_map = RequestMap {
                path: request.path.clone(),
                ..Default::default()
            };
            request_map.requests.push_back(request);
            request_set.join_inner_insert(request_map);
        }

        let mut map = HashMap::new();
        match resource_identity {
            ResourceIdentity::Drive | ResourceIdentity::Drives => {
                map.insert(
                    ResourceIdentity::Drives.to_string().to_camel_case(),
                    request_set,
                );
            }
            _ => {
                map.insert(resource_identity.to_string().to_camel_case(), request_set);
            }
        }

        Some(map)
    }
}
