use crate::common::setup_test_db; 
use crate::services::payment_service;
use crate::models::payment::{NewPayment, Payment};
use uuid::Uuid;
use validator::Validate;


#[tokio::test]
async fn test_create_payment() {
  let pool = setup_test_db().await;

  let new_payment = NewPayment {
    amount: 100.0,
    currency: "USD".to_string(),
    payment_method: PaymentMethod::CreditCard,
    payment_reason: "Test Payment".to_string(),
    status: PaymentStatus::Pending,
  };
}






#[tokio::test]
async fn test_refund_logic() {
    let pool = setup_test_db().await;
    // ... resto do teste
}
