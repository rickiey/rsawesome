use std::str::FromStr;

use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;

use crate::fmt_print::print_std; // 用来处理日期

/*
CREATE TABLE `penalty_msg` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `height` bigint(20) unsigned NOT NULL,
  `from_addr` varchar(128) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '罚款地址，一般都是是矿工',
  `to_addr` varchar(128) COLLATE utf8mb4_unicode_ci DEFAULT '',
  `amount` varchar(32) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '0' COMMENT '数额 attoFIL',
  `amount_v` decimal(30,18) NOT NULL DEFAULT '0.000000000000000000' COMMENT '数额 FIL  小数',
  `call_function` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '调用方法',
  `sub_cause` varchar(64) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '子方法，原因',
  `time_at` datetime NOT NULL COMMENT '时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `hmaf` (`height`,`from_addr`,`amount`,`call_function`,`sub_cause`)
) ENGINE=InnoDB AUTO_INCREMENT=987569 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
*/
#[derive(Debug)]
struct PenaltyMsg {
    id: Option<u64>,
    height: u64,
    from_addr: String,
    to_addr: String,
    amount: String,
    amount_v: f64,
    call_function: String,
    sub_cause: String,
    time_at: NaiveDateTime,
}

pub fn use_mysql() {
    let url = "mysql://root:passwd@127.0.0.1:3306/mine_info";
    // let pool = mysql::Pool::new(url).unwrap(); // 获取连接池
    // let mut conn = pool.get_conn().unwrap(); // 获取链接

    let pool_rst = Pool::new(Opts::from_url(url).unwrap());
    let pool = pool_rst.unwrap();

    let mut conn = pool.get_conn().expect("get connect failed");

    conn.query_iter("select * from penalty_msg order by height desc limit 10")
        .unwrap()
        .for_each(|row| {
            let r: (
                u64,
                u64,
                String,
                String,
                String,
                f64,
                String,
                String,
                NaiveDateTime,
            ) = from_row(row.unwrap());
            println!("{:?}", r);
        });

    println!("-------------------");
    let res: Vec<(
        u64,
        u64,
        String,
        String,
        String,
        f64,
        String,
        String,
        NaiveDateTime,
    )> = conn
        .query("select * from penalty_msg order by height desc limit 10")
        .unwrap();
    for r in res {
        println!("{:?}", r);
    }
    println!("-------------------");
    let mut res = conn
        .query_map(
            "select * from penalty_msg order by height desc limit 10",
            |(
                id,
                height,
                from_addr,
                to_addr,
                amount,
                amount_v,
                call_function,
                sub_cause,
                time_at,
            )| PenaltyMsg {
                id: id,
                height: height,
                from_addr: from_addr,
                to_addr: to_addr,
                amount: amount,
                amount_v: amount_v,
                call_function: call_function,
                sub_cause: sub_cause,
                time_at: time_at,
            },
        )
        .expect("Query failed.");

    for i in res {
        println!("{:?}", i)
    }

    let mut in_vet: Vec<PenaltyMsg> = vec![PenaltyMsg::new()];

    in_vet[0].height = 10;
    in_vet[0].from_addr = "from".to_string();
    in_vet[0].to_addr = "to".to_string();
    in_vet[0].call_function = "call".to_string();
    in_vet[0].amount = "10".to_string();
    in_vet[0].amount_v = 0.12345679;

    let rst = conn.exec_batch(
        r"INSERT INTO penalty_msg (`height`,`from_addr`,`to_addr`, `amount`, `amount_v`, `call_function`, `sub_cause`,`time_at`)
          VALUES (:height, :from_addr, :to_addr, :amount, :amount_v,:call_function,:sub_cause,:time_at)",
          in_vet.iter().map(|p| params! {
            "height" => p.height,
            "from_addr" => &p.from_addr,
            "to_addr" => &p.to_addr,
            "amount" => &p.amount,
            "amount_v" => &p.amount_v,
            "call_function" => &p.call_function,
            "sub_cause" => &p.sub_cause,
            "time_at" => &p.time_at,
        })
    );

    println!("{:?}", rst);
}

impl PenaltyMsg {
    pub fn new() -> PenaltyMsg {
        return PenaltyMsg {
            id: None,
            height: 0,
            from_addr: String::new(),
            to_addr: String::new(),
            amount: String::new(),
            amount_v: 0.0,
            call_function: String::new(),
            sub_cause: String::new(),
            // time_at: NaiveDateTime::from_str("2021-10-28 17:40:00").unwrap(),
            time_at: NaiveDateTime::from_timestamp(Local::now().timestamp(), 0),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_penalty() {
        let pm = PenaltyMsg::new();
        println!("{:?}", pm);
        assert_eq!(pm.amount_v, 0.0);
        assert_eq!(pm.id, None);
    }
}
