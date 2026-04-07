use crate::model::payment::{NewPayment, UpdatePayment, UpdatePaymentStatus, PaymentStatus, PaymentMethod};
use validator::Validate;

#[test]
fn test_amount_zero_invalid(){
   let payment = NewPayment {
        amount: 0.0, // ← inválido
        ..NewPayment::default()
    };

    assert!(payment.validate().is_err());
}

#[test]
fn test_amount_negative_invalid(){
    let payment = NewPayment {
        amount: -10.0, 
        ..NewPayment::default()
    };

    assert!(payment.validate().is_err());
}

#[test]
fn test_amount_valido() {
    let payment = NewPayment {
        amount: 0.01, // ← mínimo válido
        ..NewPayment::default()
    };
    assert!(payment.validate().is_ok());
}

#[test]
fn test_currency_empty_invalid() {
    let payment = NewPayment {
        amount: 10.0,
        currency: String::new(), 
        status: PaymentStatus::Completed,
        ..NewPayment::default()
    };

    let result = payment.validate();
    assert!(result.is_err());

     // Verifica se o erro é especificamente no campo "currency"
    let errors = result.unwrap_err();
    assert!(errors.field_errors().contains_key("currency"));
}

#[test]
fn test_currency_empty_valid() {
    let payment = NewPayment {
        amount: 10.0,
        currency: String::new(), 
        status: PaymentStatus::Pending, // ← status válido para permitir currency vazio
        ..NewPayment::default()
    };

    let result = payment.validate();
    assert!(result.is_ok());
}

#[test]
fn test_currency_spaces_invalid() {
    let payment = NewPayment {
        amount: 100.0,
        currency: "   ".to_string(), // ← só espaços — inválido
        status: PaymentStatus::Completed,
        ..NewPayment::default()
    };
    assert!(payment.validate().is_err());
}

#[test]
fn test_multiple_invalid_fields() {
    let payment = NewPayment {
        amount: -50.0,           // ← inválido
        currency: String::new(), // ← inválido
        status: PaymentStatus::Completed,
        ..NewPayment::default()
    };

    let result = payment.validate();
    assert!(result.is_err());

    let errors = result.unwrap_err();
    let field_errors = errors.field_errors();

    // Verifica que AMBOS os campos têm erro
    assert!(field_errors.contains_key("amount"));
    assert!(field_errors.contains_key("currency"));
}

#[test]
fn test_message_erro_amount() {
    let payment = NewPayment {
        amount: 0.0,
        ..NewPayment::default()
    };

    let errors = payment.validate().unwrap_err();
    let amount_errors = &errors.field_errors()["amount"];

    // Verifica a MESSAGE do erro (a biblioteca validator não suporta codes customizados)
    assert_eq!(
        amount_errors[0].message.as_ref().unwrap().as_ref(),
        "Amount must be greater than zero"
    );
}