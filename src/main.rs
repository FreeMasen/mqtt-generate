use mqtt::encodable::Encodable;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

type R<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Args {
    /// The directory to write the mqtt files to
    #[structopt(name = "PATH")]
    out_dir: PathBuf
}

fn main() -> R<()> {
    let args = Args::from_args();
    connack_variations(&args.out_dir)?;
    connect_variations(&args.out_dir)?;
    disconnect_variations(&args.out_dir)?;
    pingreq_variations(&args.out_dir)?;
    pingresp_variations(&args.out_dir)?;
    puback_variations(&args.out_dir)?;
    pubcomp_variations(&args.out_dir)?;
    publish_variations(&args.out_dir)?;
    pubrec_variations(&args.out_dir)?;
    pubrel_variations(&args.out_dir)?;
    suback_variations(&args.out_dir)?;
    subscribe_variations(&args.out_dir)?;
    unsuback_variations(&args.out_dir)?;
    unsubscribe_variations(&args.out_dir)?;
    Ok(())
}

fn connack_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::connack::{ConnackPacket}, control::variable_header::ConnectReturnCode};
    let mut ct = 0;
    let mut  write_packet = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("connack-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let packet = ConnackPacket::new(true, ConnectReturnCode::ConnectionAccepted);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::ConnectionAccepted);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::UnacceptableProtocolVersion);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::IdentifierRejected);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::ServiceUnavailable);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::BadUserNameOrPassword);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::NotAuthorized);
    write_packet(packet)?;
    let packet = ConnackPacket::new(false, ConnectReturnCode::Reserved(128));
    write_packet(packet)?;
    Ok(())
}

fn connect_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::connect::{ConnectPacket}, topic_name::TopicName};
    let mut ct = 0;
    let mut  write_connect = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("connect-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let mut packet = ConnectPacket::new(
        "client-id"
    );
    write_connect(packet.clone())?;
    packet.set_keep_alive(1000);
    write_connect(packet.clone())?;
    packet.set_user_name(Some("user-name".to_string()));
    write_connect(packet.clone())?;
    packet.set_will(Some((
        TopicName::new("topic").unwrap(),
        Vec::new(),
    )));
    write_connect(packet.clone())?;
    packet.set_client_identifier("other client id");
    write_connect(packet.clone())?;
    packet.set_will_retain(true);
    write_connect(packet.clone())?;
    packet.set_will_qos(2);
    write_connect(packet.clone())?;
    packet.set_clean_session(true);
    write_connect(packet.clone())?;

    Ok(())
}

fn disconnect_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::disconnect::DisconnectPacket;
    let packet = DisconnectPacket::new();
    write_packet_file(base_path.as_ref().join("disconnect.mqtt"), packet)?;
    Ok(())
}

fn pingreq_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PingreqPacket;
    let packet = PingreqPacket::new();
    write_packet_file(base_path.as_ref().join("pingreq.mqtt"), packet)?;
    Ok(())
}

fn pingresp_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PingrespPacket;
    let packet = PingrespPacket::new();
    write_packet_file(base_path.as_ref().join("pingresp.mqtt"), packet)?;
    Ok(())
}   

fn puback_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PubackPacket;
    const U16_INCREMENT: u16 = u16::MAX / 4;
    for i in 0..4 {
        write_packet_file(base_path.as_ref().join(format!("puback-{}.mqtt", i)), PubackPacket::new(i * U16_INCREMENT))?;
    }
    Ok(())
}

fn pubcomp_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PubcompPacket;
    const U16_INCREMENT: u16 = u16::MAX / 4;
    for i in 0..4 {
        write_packet_file(base_path.as_ref().join(format!("pubcomp-{}.mqtt", i)), PubcompPacket::new(i * U16_INCREMENT))?;
    }
    Ok(())
}

fn publish_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::{PublishPacket, publish::QoSWithPacketIdentifier}, topic_name::TopicName};
    let mut ct = 0;
    let mut  write_packet = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("publish-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let mut packet = PublishPacket::new(
        TopicName::new("topic").unwrap(),
        QoSWithPacketIdentifier::Level0,
        Vec::new(),
    );
    write_packet(packet.clone())?;
    packet.set_dup(true);
    write_packet(packet.clone())?;
    packet.set_qos(QoSWithPacketIdentifier::Level1(1));
    write_packet(packet.clone())?;
    packet.set_qos(QoSWithPacketIdentifier::Level2(1));
    write_packet(packet.clone())?;
    packet.set_payload(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
        eiusmod tempor incididunt ut labore et dolore magna aliqua."
        .repeat(10).as_bytes().to_vec()
    );
    write_packet(packet.clone())?;
    Ok(())
}

fn pubrec_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PubrecPacket;
    const U16_INCREMENT: u16 = u16::MAX / 4;
    for i in 0..4 {
        write_packet_file(base_path.as_ref().join(format!("pubrec-{}.mqtt", i)), PubrecPacket::new(i * U16_INCREMENT))?;
    }
    Ok(())
}

fn pubrel_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::PubrelPacket;
    const U16_INCREMENT: u16 = u16::MAX / 4;
    for i in 0..4 {
        write_packet_file(base_path.as_ref().join(format!("pubrel-{}.mqtt", i)), PubrelPacket::new(i * U16_INCREMENT))?;
    }
    Ok(())
}

fn suback_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::packet::{SubackPacket, suback::SubscribeReturnCode};
    let mut ct = 0;
    let mut  write_packet = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("suback-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let packet = SubackPacket::new(
        1, Vec::new(),
    );
    write_packet(packet)?;
    let packet = SubackPacket::new(
        2, vec![
            SubscribeReturnCode::MaximumQoSLevel0,
        ],
    );
    write_packet(packet)?;
    let packet = SubackPacket::new(
        3, vec![
            SubscribeReturnCode::MaximumQoSLevel0,
            SubscribeReturnCode::MaximumQoSLevel1,
        ],
    );
    write_packet(packet)?;
    let packet = SubackPacket::new(
        4, vec![
            SubscribeReturnCode::MaximumQoSLevel0,
            SubscribeReturnCode::MaximumQoSLevel1,
            SubscribeReturnCode::MaximumQoSLevel2,
        ],
    );
    write_packet(packet)?;
    let packet = SubackPacket::new(
        5, vec![
            SubscribeReturnCode::MaximumQoSLevel0,
            SubscribeReturnCode::MaximumQoSLevel1,
            SubscribeReturnCode::MaximumQoSLevel2,
            SubscribeReturnCode::Failure,
        ],
    );
    write_packet(packet)?;
    Ok(())
}

fn subscribe_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::SubscribePacket, topic_filter::TopicFilter, qos::QualityOfService};
    let mut ct = 0;
    let mut  write_packet = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("subscribe-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let packet = SubscribePacket::new(
        1, Vec::new(),
    );
    write_packet(packet)?;
    let packet = SubscribePacket::new(
        2, vec![
            (TopicFilter::new("#").unwrap(), QualityOfService::Level0)
        ],
    );
    write_packet(packet)?;
    let packet = SubscribePacket::new(
        3, vec![
            (TopicFilter::new("#").unwrap(), QualityOfService::Level0),
            (TopicFilter::new("topic/filter").unwrap(), QualityOfService::Level1),
        ],
    );
    write_packet(packet)?;
    let packet = SubscribePacket::new(
        4, vec![
            (TopicFilter::new("#").unwrap(), QualityOfService::Level0),
            (TopicFilter::new("topic/filter").unwrap(), QualityOfService::Level1),
            (TopicFilter::new("topic/+/filter").unwrap(), QualityOfService::Level2),
        ],
    );
    write_packet(packet)?;
    let packet = SubscribePacket::new(
        5, vec![
            (TopicFilter::new("#").unwrap(), QualityOfService::Level0),
            (TopicFilter::new("topic/filter").unwrap(), QualityOfService::Level1),
            (TopicFilter::new("topic/+/filter").unwrap(), QualityOfService::Level2),
            (TopicFilter::new("topic/+/filter/#").unwrap(), QualityOfService::Level2),
        ],
    );
    write_packet(packet)?;
    Ok(())
}

fn unsuback_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::UnsubackPacket};
    const U16_INCREMENT: u16 = u16::MAX / 4;
    for i in 0..4 {
        write_packet_file(base_path.as_ref().join(format!("unsuback-{}.mqtt", i)), UnsubackPacket::new(i * U16_INCREMENT))?;
    }
    Ok(())
}

fn unsubscribe_variations<P: AsRef<Path>>(base_path: P) -> R<()> {
    use mqtt::{packet::UnsubscribePacket, topic_filter::TopicFilter};
    let mut ct = 0;
    let mut  write_packet = |packet| {
        ct += 1;
        write_packet_file(
            base_path.as_ref().join(format!("unsubscribe-{:02}.mqtt", ct)),
            packet
        )?;
        R::Ok(())
    };
    let packet = UnsubscribePacket::new(
        1, Vec::new(),
    );
    write_packet(packet)?;
    let packet = UnsubscribePacket::new(
        2, vec![
            TopicFilter::new("#").unwrap()
        ],
    );
    write_packet(packet)?;
    let packet = UnsubscribePacket::new(
        3, vec![
            TopicFilter::new("#").unwrap(),
            TopicFilter::new("topic/filter").unwrap(),
        ],
    );
    write_packet(packet)?;
    let packet = UnsubscribePacket::new(
        4, vec![
            TopicFilter::new("#").unwrap(),
            TopicFilter::new("topic/filter").unwrap(),
            TopicFilter::new("topic/+/filter").unwrap(),
        ],
    );
    write_packet(packet)?;
    let packet = UnsubscribePacket::new(
        5, vec![
            TopicFilter::new("#").unwrap(),
            TopicFilter::new("topic/filter").unwrap(),
            TopicFilter::new("topic/+/filter").unwrap(),
            TopicFilter::new("topic/+/filter/#").unwrap(),
        ],
    );
    write_packet(packet)?;
    Ok(())
}

fn write_packet_file<P: AsRef<Path>>(path: P, packet: impl Encodable) -> R<()> {
    let mut f = std::fs::File::create(&path)?;
    packet.encode(&mut f)?;
    Ok(())
}
