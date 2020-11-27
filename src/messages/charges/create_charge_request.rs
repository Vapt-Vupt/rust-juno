use crate::messages::AbstractRequest;
use reqwest::Method;
use serde_json::*;
use crate::utils::*;

// https://dev.juno.com.br/api/v2#operation/createCharge

pub struct CreateChargeRequest {
    pub resource_token: String,
    pub parameters: Value,
}

impl AbstractRequest for CreateChargeRequest {
    fn resource_token(&self) -> Option<&String> {
        Some(&self.resource_token)
    }
    
    fn http_method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("charges")
    }

    fn data(&self) -> Value {
        let params = self.parameters.only_or_die(&[
            "charge",
            "billing",
        ]);

        let charge = params["charge"].only(&[
            "description",
            "references",
            "totalAmount",
            "amount",
            "dueDate",
            "installments",
            "maxOverdueDays",
            "fine",
            "interest",
            "discountAmount",
            "discountDays",
            "paymentTypes",
            "paymentAdvance",
            "feeSchemaToken",
            "dueDate",
            "split",
        ]);

        let billing = params["billing"].only(&[
            "name",
            "document",
            "email",
            "secondaryEmail",
            "phone",
            "birthDate",
            "notify",
        ]);

        charge.validate_or_die(&[
            "description",
            "amount"
        ]);

        billing.validate_or_die(&[
            "name",
            "document"
        ]);

        json!({
            "charge": charge,
            "billing": billing,
        })
    }
}

