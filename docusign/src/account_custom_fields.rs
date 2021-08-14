use anyhow::Result;

use crate::Client;

pub struct AccountCustomFields {
    client: Client,
}

impl AccountCustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountCustomFields { client }
    }

    /**
     * Gets a list of custom fields.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/custom_fields` endpoint.
     *
     * This method returns a list of the envelope and document custom fields associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str) -> Result<crate::types::AccountCustomFields> {
        let url = format!(
            "/v2.1/accounts/{}/custom_fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Creates an account custom field.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/custom_fields` endpoint.
     *
     * This method creates a custom field and makes it available for all new envelopes associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- (Optional) When set to **true**, the new custom field is applied to all of the templates on the account.
     */
    pub async fn post(
        &self,
        account_id: &str,
        apply_to_templates: &str,
        body: &crate::types::CustomField,
    ) -> Result<crate::types::AccountCustomFields> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push(format!("apply_to_templates={}", apply_to_templates));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/custom_fields?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Updates an account custom field.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/custom_fields/{customFieldId}` endpoint.
     *
     * This method updates an existing account custom field.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_field_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        custom_field_id: &str,
        apply_to_templates: &str,
        body: &crate::types::CustomField,
    ) -> Result<crate::types::AccountCustomFields> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push(format!("apply_to_templates={}", apply_to_templates));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/custom_fields/{}?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&custom_field_id.to_string()),
            query
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Deletes an account custom field.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/custom_fields/{customFieldId}` endpoint.
     *
     * This method deletes an existing account custom field.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `custom_field_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `apply_to_templates: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        custom_field_id: &str,
        apply_to_templates: &str,
    ) -> Result<()> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !apply_to_templates.is_empty() {
            query_args.push(format!("apply_to_templates={}", apply_to_templates));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/custom_fields/{}?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&custom_field_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }
}