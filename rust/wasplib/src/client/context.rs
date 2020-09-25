// encapsulates standard host entities into a simple interface

use super::host::set_string;
use super::immutable::ScImmutableMap;
use super::immutable::ScImmutableStringArray;
use super::keys::key_log;
use super::keys::key_trace;
use super::mutable::ScMutableMap;
use super::mutable::ScMutableString;

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScAccount {
    account: ScImmutableMap,
}

impl ScAccount {
    pub fn balance(&self, color: &str) -> i64 {
        self.account.get_map("balance").get_int(color).value()
    }

    pub fn colors(&self) -> ScImmutableStringArray {
        self.account.get_string_array("colors")
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScContract {
    contract: ScImmutableMap,
}

impl ScContract {
    pub fn address(&self) -> String {
        self.contract.get_string("address").value()
    }

    pub fn color(&self) -> String {
        self.contract.get_string("color").value()
    }

    pub fn description(&self) -> String {
        self.contract.get_string("description").value()
    }

    pub fn id(&self) -> String {
        self.contract.get_string("id").value()
    }

    pub fn name(&self) -> String {
        self.contract.get_string("name").value()
    }

    pub fn owner(&self) -> String {
        self.contract.get_string("owner").value()
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScEvent {
    event: ScMutableMap,
}

impl ScEvent {
    pub fn code(&self, code: i64) {
        self.event.get_int("code").set_value(code);
    }

    pub fn contract(&self, contract: &str) {
        self.event.get_string("contract").set_value(contract);
    }

    pub fn delay(&self, delay: i64) {
        self.event.get_int("delay").set_value(delay);
    }

    pub fn function(&self, function: &str) {
        self.event.get_string("function").set_value(function);
    }

    pub fn params(&self) -> ScMutableMap {
        self.event.get_map("params")
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\


#[derive(Copy, Clone)]
pub struct ScLog {
    log: ScMutableMap,
}

impl ScLog {
    pub fn append(&self, timestamp: i64, data: &[u8]) {
        self.log.get_int("timestamp").set_value(timestamp);
        self.log.get_bytes("data").set_value(data);
    }

    pub fn length(&self) -> i32 {
        self.log.get_int("length").value() as i32
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScRequest {
    request: ScImmutableMap,
}

impl ScRequest {
    pub fn address(&self) -> String {
        self.request.get_string("address").value()
    }

    pub fn balance(&self, color: &str) -> i64 {
        self.request.get_map("balance").get_int(color).value()
    }

    pub fn colors(&self) -> ScImmutableStringArray {
        self.request.get_string_array("colors")
    }

    pub fn hash(&self) -> String {
        self.request.get_string("hash").value()
    }

    pub fn id(&self) -> String {
        self.request.get_string("id").value()
    }

    pub fn params(&self) -> ScImmutableMap {
        self.request.get_map("params")
    }

    pub fn timestamp(&self) -> i64 {
        self.request.get_int("timestamp").value()
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScTransfer {
    transfer: ScMutableMap,
}

impl ScTransfer {
    pub fn address(&self, address: &str) {
        self.transfer.get_string("address").set_value(address);
    }

    pub fn amount(&self, amount: i64) {
        self.transfer.get_int("amount").set_value(amount);
    }

    pub fn color(&self, color: &str) {
        self.transfer.get_string("color").set_value(color);
    }
}


// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScUtility {
    utility: ScMutableMap,
}

impl ScUtility {
    pub fn hash(&self, value: &str) -> String {
        let hash = self.utility.get_string("hash");
        hash.set_value(value);
        hash.value()
    }
}

// \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\ // \\

#[derive(Copy, Clone)]
pub struct ScContext {
    root: ScMutableMap,
}

impl ScContext {
    pub fn new() -> ScContext {
        ScContext { root: ScMutableMap::new(1) }
    }

    pub fn account(&self) -> ScAccount {
        ScAccount { account: self.root.get_map("account").immutable() }
    }

    pub fn contract(&self) -> ScContract {
        ScContract { contract: self.root.get_map("contract").immutable() }
    }

    pub fn error(&self) -> ScMutableString {
        self.root.get_string("error")
    }

    pub fn event(&self, contract: &str, function: &str, delay: i64) -> ScMutableMap {
        let events = self.root.get_map_array("events");
        let evt = ScEvent { event: events.get_map(events.length()) };
        evt.contract(contract);
        evt.function(function);
        evt.delay(delay);
        evt.params()
    }

    // just for compatibility with old hardcoded SCs
    pub fn event_with_code(&self, contract: &str, code: i64, delay: i64) -> ScMutableMap {
        let events = self.root.get_map_array("events");
        let evt = ScEvent { event: events.get_map(events.length()) };
        evt.contract(contract);
        evt.code(code);
        evt.delay(delay);
        evt.params()
    }

    pub fn log(&self, text: &str) {
        set_string(1, key_log(), text)
    }

    pub fn random(&self, max: i64) -> i64 {
        let rnd = self.root.get_int("random").value();
        (rnd as u64 % max as u64) as i64
    }

    pub fn request(&self) -> ScRequest {
        ScRequest { request: self.root.get_map("request").immutable() }
    }

    pub fn state(&self) -> ScMutableMap {
        self.root.get_map("state")
    }

    pub fn timestamped_log(&self, key: &str) -> ScLog {
        ScLog { log: self.root.get_map("logs").get_map(key) }
    }
    pub fn trace(&self, text: &str) {
        set_string(1, key_trace(), text)
    }

    pub fn transfer(&self, address: &str, color: &str, amount: i64) {
        let transfers = self.root.get_map_array("transfers");
        let xfer = ScTransfer { transfer: transfers.get_map(transfers.length()) };
        xfer.address(address);
        xfer.color(color);
        xfer.amount(amount);
    }

    pub fn utility(&self) -> ScUtility {
        ScUtility { utility: self.root.get_map("utility") }
    }
}
