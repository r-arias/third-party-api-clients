use anyhow::Result;

use crate::Client;

pub struct Schemas {
    client: Client,
}

impl Schemas {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Schemas { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/schemas` endpoint.
     *
     * Retrieves all schemas for a customer.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn directory_list(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
    ) -> Result<crate::types::Schemas> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/schemas` endpoint.
     *
     * Creates a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn directory_insert(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        body: &crate::types::Schema,
    ) -> Result<crate::types::Schema> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            query_
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Retrieves a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn directory_get(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        schema_key: &str,
    ) -> Result<crate::types::Schema> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas/{}?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&schema_key.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Updates a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn directory_update(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        schema_key: &str,
        body: &crate::types::Schema,
    ) -> Result<crate::types::Schema> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas/{}?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&schema_key.to_string()),
            query_
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Deletes a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn directory_delete(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        schema_key: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas/{}?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&schema_key.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Patches a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn directory_patch(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        customer_id: &str,
        schema_key: &str,
        body: &crate::types::Schema,
    ) -> Result<crate::types::Schema> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/customer/{}/schemas/{}?{}",
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            crate::progenitor_support::encode_path(&schema_key.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}