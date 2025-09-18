use crate::spot::ws::message::kline::KlineIntervalTopic;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Topic {
    AccountDeals,
    AccountOrders,
    AccountUpdate,
    Deals(DealsTopic),
    Kline(KlineTopic),
    Depth(DepthTopic),
    BookTicker(String),
    BookTickerBatch(String)
}

impl Topic {
    pub fn requires_auth(&self) -> bool {
        match self {
            Topic::AccountDeals => true,
            Topic::AccountOrders => true,
            Topic::AccountUpdate => true,
            Topic::Deals(_) => false,
            Topic::Kline(_) => false,
            Topic::Depth(_) => false,
            Topic::BookTicker(_) => false,
            Topic::BookTickerBatch(_) => false,
        }
    }

    pub fn to_topic_subscription_string(&self) -> String {
        match self {
            Topic::AccountDeals => "spot@private.deals.v3.api.pb".to_string(),
            Topic::AccountOrders => "spot@private.orders.v3.api.pb".to_string(),
            Topic::AccountUpdate => "spot@private.account.v3.api.pb".to_string(),
            Topic::Deals(deals_topic) => format!(
                "spot@public.aggre.deals.v3.api.pb@10ms@{symbol}",
                symbol = deals_topic.symbol
            ),
            Topic::Kline(kline_topic) => format!(
                "spot@public.kline.v3.api.pb@{symbol}@{interval}",
                symbol = kline_topic.symbol,
                interval = kline_topic.interval.as_ref()
            ),
            Topic::Depth(depth_topic) => format!(
                "spot@public.aggre.depth.v3.api.pb@10@{symbol}",
                symbol = depth_topic.symbol
            ),
            Topic::BookTicker(symbol) => format!(
                "spot@public.aggre.bookTicker.v3.api.pb@10ms@{symbol}"
            ),
            Topic::BookTickerBatch(symbol) => format!(
                "spot@public.bookTicker.batch.v3.api.pb@{symbol}"
            ),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DealsTopic {
    pub symbol: String,
}

impl DealsTopic {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KlineTopic {
    pub symbol: String,
    pub interval: KlineIntervalTopic,
}

impl KlineTopic {
    pub fn new(symbol: String, interval: KlineIntervalTopic) -> Self {
        Self { symbol, interval }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DepthTopic {
    pub symbol: String,
}

impl DepthTopic {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}
