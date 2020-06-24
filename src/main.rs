use rusoto_core::Region;
use rusoto_route53::{ChangeResourceRecordSetsRequest, ChangeBatch, Change, ResourceRecordSet, ResourceRecord, Route53, Route53Client};
use std::env;

use hyper::body::HttpBody as _;
use hyper::{Client, Uri};
use std::str;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage dyndns53 <hosted_zone_id> <domain>");
        return;
    }
    let hosted_zone_id = &args[0];
    if hosted_zone_id.is_empty() {
        println!("Hosted zone cannot be empty");
        return;
    }
    let domain = &args[1];
    if domain.is_empty() {
        println!("Domain cannot be empty");
        return;
    }

    match get_ip().await {
        Ok(ip) => {
            println!("successfully retrieved ip {}", ip);
            match set_ip_in_route53(hosted_zone_id, domain, &ip, 300).await {
                Ok(_) => {
                    println!("Succesfully set IP");
                },
                Err(error) => {
                    println!("Failed setting ip: {:?}", error);
                }
            }
        },
        Err(error) => {
            println!("Failed retrieveing IP: {:?}", error);
        }
    }
}

async fn set_ip_in_route53(hosted_zone_id: &str, domain_name: &str, ip: &str, ttl: i64) -> Result<String, String> {
    let client = Route53Client::new(Region::UsEast1);

    let mut changes = Vec::new();
    let mut change = Change::default();
    change.action = "UPSERT".to_string();
    change.resource_record_set = ResourceRecordSet::default();
    change.resource_record_set.name = domain_name.to_string();
    change.resource_record_set.type_ = "A".to_string();
    change.resource_record_set.ttl = Option::from(ttl);
    let mut resource_records = Vec::new();
    let mut resource_record = ResourceRecord::default();
    resource_record.value = ip.to_string();
    resource_records.push(resource_record);
    change.resource_record_set.resource_records = Option::from(resource_records);

    changes.push(change);

    let mut change_batch = ChangeBatch::default();
    change_batch.changes = changes;

    let mut request = ChangeResourceRecordSetsRequest::default();
    request.hosted_zone_id = format!("/hostedzone/{}", hosted_zone_id);
    request.change_batch = change_batch;
    return match client.change_resource_record_sets(request).await {
        Ok(output) => {
            println!("success {:?}", output.change_info);
            Ok("Updated recordset".to_string())
        },
        Err(error) => {
            println!("Error: {:?}", error);
            Err("Failed setting record".to_string())
        }
    };
}

async fn get_ip() -> Result<String, String> {
    let client = Client::new();
    let uri = "http://bot.whatismyipaddress.com/".parse::<Uri>().unwrap();

    return match client.get(uri).await {
        Ok(mut resp) => {
            let body = resp.body_mut();
            let mut chunks = String::new();
            while let Some(chunk) = body.data().await {
                match chunk {
                    Ok(content) => {
                        chunks.push_str(str::from_utf8(&content).unwrap());
                    },
                    Err(error) => {
                        println!("Error: {:?}", error);
                    }
                }
            }
            Ok(chunks.trim().to_string())
        },
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error.to_string())
        }
    }
}
