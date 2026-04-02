use crate::model::{NewPayment, PaymentStatus, PaymentMethod, UpdatePayment, UpdatePaymentStatus};


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


impl Default for UpdatePayment{
  fn default()-> Self {
    UpdatePayment{
      payment_method: PaymentMethod::Money,
      currency: String::from("BRL"),
      status: PaymentStatus::Completed,
    }
  }
}

let update_payment = UpdatePayment::default();

impl Default for UpdatePaymentStatus{
  fn default()-> Self {
    UpdatePaymentStatus{
      status: PaymentStatus::Canceled,
    }
  }
}