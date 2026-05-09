use pyo3::prelude::*;

use crate::qaaccount::QA_Account;

/// Wrapper for QA_Account to make it Python-compatible
#[pyclass]
pub struct AccountWrapper(pub QA_Account);

#[pymethods]
impl AccountWrapper {
    #[new]
    fn new(
        account_cookie: &str,
        portfolio_cookie: &str,
        user_cookie: &str,
        init_cash: f64,
        environment: &str,
    ) -> Self {
        AccountWrapper(QA_Account::new(
            account_cookie,
            portfolio_cookie,
            user_cookie,
            init_cash,
            false,
            environment,
        ))
    }

    fn get_balance(&mut self) -> f64 {
        self.0.get_balance()
    }

    fn get_cash(&mut self) -> f64 {
        self.0.get_cash()
    }

    fn buy_open(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.buy_open(code, amount, time, price).is_ok()
    }

    fn sell_open(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.sell_open(code, amount, time, price).is_ok()
    }

    fn buy_close(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.buy_close(code, amount, time, price).is_ok()
    }

    fn sell_close(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.sell_close(code, amount, time, price).is_ok()
    }

    fn buy(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.buy(code, amount, time, price).is_ok()
    }

    fn sell(&mut self, code: &str, amount: f64, time: &str, price: f64) -> bool {
        self.0.sell(code, amount, time, price).is_ok()
    }

    fn init_h(&mut self, code: &str) {
        self.0.init_h(code);
    }

    fn get_volume_long(&mut self, code: &str) -> f64 {
        self.0.get_volume_long(code)
    }

    fn get_volume_short(&mut self, code: &str) -> f64 {
        self.0.get_volume_short(code)
    }

    fn on_price_change(&mut self, code: String, price: f64, datetime: String) {
        self.0.on_price_change(code, price, datetime);
    }

    fn get_floatprofit(&mut self) -> f64 {
        self.0.get_floatprofit()
    }

    fn settle(&mut self) {
        self.0.settle();
    }

    fn get_qifi_slice(&mut self) -> String {
        let slice = self.0.get_qifi_slice();
        serde_json::to_string(&slice).unwrap_or_default()
    }

    fn get_slice(&mut self) -> String {
        let slice = self.0.get_slice();
        serde_json::to_string(&slice).unwrap_or_default()
    }
}

/// Python module definition
#[pymodule]
pub fn quantaxis_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AccountWrapper>()?;
    Ok(())
}