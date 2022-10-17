// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;

register_client!(ApplicationsRequest,);
register_client!(ApplicationsIdRequest, ());

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, applications_id: ID) -> ApplicationsIdRequest<'a, Client> {
        ApplicationsIdRequest::new(applications_id.as_ref(), self.client)
    }

    post!({
        doc: "Create application",
        name: create_application,
        response: serde_json::Value,
        path: "/applications",
        has_body: true
    });
    get!({
        doc: "List applications",
        name: list_application,
        response: serde_json::Value,
        path: "/applications",
        has_body: false
    });
    get!({
        doc: "Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/applications/microsoft.graph.delta()",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/applications/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/applications/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/applications/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> ApplicationsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete application",
        name: delete_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get application",
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update application",
        name: update_application,
        response: serde_json::Value,
        path: "/applications/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf",
        has_body: false
    });
    post!({
        doc: "Create extensionProperty (directory extension)",
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        has_body: true
    });
    get!({
        doc: "List extensionProperties (directory extensions)",
        name: list_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_extension_properties_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property extensionProperties for applications",
        name: delete_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: false
    });
    get!({
        doc: "Get extensionProperties from applications",
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: true
    });
    post!({
        doc: "Create federatedIdentityCredential",
        name: create_federated_identity_credentials,
        response: serde_json::Value,
        path: "/applications/{{RID}}/federatedIdentityCredentials",
        has_body: true
    });
    get!({
        doc: "List federatedIdentityCredentials",
        name: list_federated_identity_credentials,
        response: serde_json::Value,
        path: "/applications/{{RID}}/federatedIdentityCredentials",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_federated_identity_credentials_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/federatedIdentityCredentials/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property federatedIdentityCredentials for applications",
        name: delete_federated_identity_credentials,
        response: NoContent,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: [ federated_identity_credential_id ],
        has_body: false
    });
    get!({
        doc: "Get federatedIdentityCredentials from applications",
        name: get_federated_identity_credentials,
        response: serde_json::Value,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: [ federated_identity_credential_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property federatedIdentityCredentials in applications",
        name: update_federated_identity_credentials,
        response: serde_json::Value,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: [ federated_identity_credential_id ],
        has_body: true
    });
    get!({
        doc: "Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_home_realm_discovery_policies_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$count",
        has_body: false
    });
    get!({
        doc: "Get homeRealmDiscoveryPolicies from applications",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/{{id}}",
        params: [ home_realm_discovery_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get logo for application from applications",
        name: get_logo,
        response: serde_json::Value,
        path: "/applications/{{RID}}/logo",
        has_body: false
    });
    put!({
        doc: "Update logo for application in applications",
        name: update_logo,
        response: NoContent,
        path: "/applications/{{RID}}/logo",
        has_body: true
    });
    post!({
        doc: "Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.addKey",
        has_body: true
    });
    post!({
        doc: "Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.addPassword",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action removeKey",
        name: remove_key,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.removeKey",
        has_body: true
    });
    post!({
        doc: "Invoke action removePassword",
        name: remove_password,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.removePassword",
        has_body: true
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
    post!({
        doc: "Invoke action setVerifiedPublisher",
        name: set_verified_publisher,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.setVerifiedPublisher",
        has_body: true
    });
    post!({
        doc: "Invoke action unsetVerifiedPublisher",
        name: unset_verified_publisher,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.unsetVerifiedPublisher",
        has_body: false
    });
    get!({
        doc: "Get owners from applications",
        name: list_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_owners_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to owners for applications",
        name: create_ref_owners,
        response: NoContent,
        path: "/applications/{{RID}}/owners/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of owners from applications",
        name: list_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$ref",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_app_role_assignment_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.appRoleAssignment",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_app_role_assignment_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.appRoleAssignment/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_endpoint_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.endpoint",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.endpoint/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.servicePrincipal",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.servicePrincipal/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.user",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_user_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/microsoft.graph.user/$count",
        has_body: false
    });
    delete!({
        doc: "Delete ref of navigation property owners for applications",
        name: delete_ref_owners,
        response: NoContent,
        path: "/applications/{{RID}}/owners/{{id}}/$ref",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: get_directory_object_item_as_app_role_assignment_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/{{id}}/microsoft.graph.appRoleAssignment",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: get_directory_object_item_as_endpoint_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/{{id}}/microsoft.graph.endpoint",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/{{id}}/microsoft.graph.servicePrincipal",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/{{id}}/microsoft.graph.user",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "List assigned tokenIssuancePolicies",
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_token_issuance_policies_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to tokenIssuancePolicies for applications",
        name: create_ref_token_issuance_policies,
        response: NoContent,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        has_body: true
    });
    get!({
        doc: "List assigned tokenIssuancePolicies",
        name: list_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        has_body: false
    });
    delete!({
        doc: "Delete ref of navigation property tokenIssuancePolicies for applications",
        name: delete_ref_token_issuance_policies,
        response: NoContent,
        path: "/applications/{{RID}}/tokenIssuancePolicies/{{id}}/$ref",
        params: [ token_issuance_policy_id ],
        has_body: false
    });
    get!({
        doc: "List assigned tokenLifetimePolicy",
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_token_lifetime_policies_count,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to tokenLifetimePolicies for applications",
        name: create_ref_token_lifetime_policies,
        response: NoContent,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        has_body: true
    });
    get!({
        doc: "List assigned tokenLifetimePolicy",
        name: list_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        has_body: false
    });
    delete!({
        doc: "Delete ref of navigation property tokenLifetimePolicies for applications",
        name: delete_ref_token_lifetime_policies,
        response: NoContent,
        path: "/applications/{{RID}}/tokenLifetimePolicies/{{id}}/$ref",
        params: [ token_lifetime_policy_id ],
        has_body: false
    });
}
