-- Вставка данных
INSERT INTO orders (
    order_uid,
    track_number,
    entry,
    delivery,
    payment,
    items,
    locale,
    internal_signature,
    customer_id,
    delivery_service,
    shardkey,
    sm_id,
    date_created,
    oof_shard
) VALUES (
             'b563feb7b2b84b6test',
             'WBILMTESTTRACK',
             'WBIL',
             '{"name": "Test Testov", "phone": "+9720000000", "zip": "2639809", "city": "Kiryat Mozkin", "address": "Ploshad Mira 15", "region": "Kraiot", "email": "test@gmail.com"}',
             '{"transaction": "b563feb7b2b84b6test", "request_id": "", "currency": "USD", "provider": "wbpay", "amount": 1817, "payment_dt": 1637907727, "bank": "alpha", "delivery_cost": 1500, "goods_total": 317, "custom_fee": 0}',
             '[{"chrt_id": 9934930, "track_number": "WBILMTESTTRACK", "price": 453, "rid": "ab4219087a764ae0btest", "name": "Mascaras", "sale": 30, "size": "0", "total_price": 317, "nm_id": 2389212, "brand": "Vivienne Sabo", "status": 202}]',
             'en',
             '',
             'test',
             'meest',
             '9',
             99,
             '2021-11-26T06:22:19Z',
             '1'
         );
