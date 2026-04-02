use validator::{Validate, ValidationError};
use crate::models::payment_model::{NewPayment, PaymentMethod, PaymentStatus};

impl Validate for NewPayment {
  fn validate(&self)-> Result<(), validator::ValidationErrors>{
    let mut errors = validator::ValidationErrors::new();

    if self.amount <= 0.0 {
      errors.add("amount", ValidationError::new("Amount must be greater than zero"));
    }

    if self.currency.trim().is_empty(){
      errors.add("currency", ValidationError::new("Currency cannot be empty"));
    }

    if self.payment_method != PaymentMethod::CreditCard && self.payment_method != PaymentMethod::DebitCard && self.payment_method != PaymentMethod::Money && self.payment_method != PaymentMethod::Pix {
      errors.add("payment_method", ValidationError::new("Invalid payment method"));
    }

    if self.payment_reason.trim().is_empty() {
      errors.add("payment_reason", ValidationError::new("Payment reason cannot be empty"));
    }

    if self.status != PaymentStatus::Pending && self.status != PaymentStatus::Completed && self.status != PaymentStatus::Canceled {
      errors.add("status", ValidationError::new("Invalid payment status"));
    }

    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }
}