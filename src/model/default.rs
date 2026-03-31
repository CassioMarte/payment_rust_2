use crate::model::{NewPayment, PaymentStatus, PaymentMethod};


impl Default for NewPayment {
  fn default()-> Self {
    NewPayment{
      amount: 0.0,
      currency: String::from("BRL"),
      payment_method: PaymentMethod::Money,
      payment_reason: String::from(""),
      status: PaymentStatus::Pending,
    }
  }
}

let new_payment = NewPayment::default();