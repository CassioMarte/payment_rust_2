use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

enum PaymentStatus {
    Pending,
    Completed,
    Canceled,
}

enum PaymentMethod {
    CreditCard,
    DebitCard,
    Money,
    Pix,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
 pub id: u32,
 pub uuid: Uuid,
 pub amount: f64,
 pub currency: String,
 pub payment_method: PaymentMethod,
 pub payment_reason: String,
 pub status: PaymentStatus,
 pub created_at: NaiveDateTime,
 pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct NewPayment {
    pub amount: f64,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub payment_reason: String,
    pub status: PaymentStatus,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePayment {
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
}



#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePaymentStatus {
    pub status: PaymentStatus,
}
