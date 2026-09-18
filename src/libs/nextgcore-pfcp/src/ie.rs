//! PFCP Information Elements
//!
//! IE types and encoding/decoding for PFCP protocol as specified in 3GPP TS 29.244.

use crate::error::{PfcpError, PfcpResult};
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// PFCP IE Type values (TS 29.244 Section 8.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum IeType {
    Reserved0 = 0,
    CreatePdr = 1,
    Pdi = 2,
    CreateFar = 3,
    ForwardingParameters = 4,
    DuplicatingParameters = 5,
    CreateUrr = 6,
    CreateQer = 7,
    CreatedPdr = 8,
    UpdatePdr = 9,
    UpdateFar = 10,
    UpdateForwardingParameters = 11,
    UpdateBar = 12,
    UpdateUrr = 13,
    UpdateQer = 14,
    RemovePdr = 15,
    RemoveFar = 16,
    RemoveUrr = 17,
    RemoveQer = 18,
    Cause = 19,
    SourceInterface = 20,
    FTeid = 21,
    NetworkInstance = 22,
    SdfFilter = 23,
    ApplicationId = 24,
    GateStatus = 25,
    Mbr = 26,
    Gbr = 27,
    QerCorrelationId = 28,
    Precedence = 29,
    TransportLevelMarking = 30,
    VolumeThreshold = 31,
    TimeThreshold = 32,
    MonitoringTime = 33,
    SubsequentVolumeThreshold = 34,
    SubsequentTimeThreshold = 35,
    InactivityDetectionTime = 36,
    ReportingTriggers = 37,
    RedirectInformation = 38,
    ReportType = 39,
    OffendingIe = 40,
    ForwardingPolicy = 41,
    DestinationInterface = 42,
    UpFunctionFeatures = 43,
    ApplyAction = 44,
    DownlinkDataServiceInformation = 45,
    DownlinkDataNotificationDelay = 46,
    DlBufferingDuration = 47,
    DlBufferingSuggestedPacketCount = 48,
    PfcpSmreqFlags = 49,
    PfcpSrrspFlags = 50,
    LoadControlInformation = 51,
    SequenceNumber = 52,
    Metric = 53,
    OverloadControlInformation = 54,
    Timer = 55,
    PdrId = 56,
    FSeid = 57,
    ApplicationIdsPfds = 58,
    PfdContext = 59,
    NodeId = 60,
    PfdContents = 61,
    MeasurementMethod = 62,
    UsageReportTrigger = 63,
    MeasurementPeriod = 64,
    FqCsid = 65,
    VolumeMeasurement = 66,
    DurationMeasurement = 67,
    ApplicationDetectionInformation = 68,
    TimeOfFirstPacket = 69,
    TimeOfLastPacket = 70,
    QuotaHoldingTime = 71,
    DroppedDlTrafficThreshold = 72,
    VolumeQuota = 73,
    TimeQuota = 74,
    StartTime = 75,
    EndTime = 76,
    QueryUrr = 77,
    UsageReportSmr = 78,
    UsageReportSdr = 79,
    UsageReportSrr = 80,
    UrrId = 81,
    LinkedUrrId = 82,
    DownlinkDataReport = 83,
    OuterHeaderCreation = 84,
    CreateBar = 85,
    UpdateBarSmr = 86,
    RemoveBar = 87,
    BarId = 88,
    CpFunctionFeatures = 89,
    UsageInformation = 90,
    ApplicationInstanceId = 91,
    FlowInformation = 92,
    UeIpAddress = 93,
    PacketRate = 94,
    OuterHeaderRemoval = 95,
    RecoveryTimeStamp = 96,
    DlFlowLevelMarking = 97,
    HeaderEnrichment = 98,
    ErrorIndicationReport = 99,
    MeasurementInformation = 100,
    NodeReportType = 101,
    UserPlanePathFailureReport = 102,
    RemoteGtpUPeer = 103,
    UrSeqn = 104,
    UpdateDuplicatingParameters = 105,
    ActivatePredefinedRules = 106,
    DeactivatePredefinedRules = 107,
    FarId = 108,
    QerId = 109,
    OciFlags = 110,
    PfcpAssociationReleaseRequest = 111,
    GracefulReleasePeriod = 112,
    PdnType = 113,
    FailedRuleId = 114,
    TimeQuotaMechanism = 115,
    Reserved116 = 116,
    UserPlaneInactivityTimer = 117,
    AggregatedUrrs = 118,
    Multiplier = 119,
    AggregatedUrrId = 120,
    SubsequentVolumeQuota = 121,
    SubsequentTimeQuota = 122,
    Rqi = 123,
    Qfi = 124,
    QueryUrrReference = 125,
    AdditionalUsageReportsInformation = 126,
    CreateTrafficEndpoint = 127,
    CreatedTrafficEndpoint = 128,
    UpdateTrafficEndpoint = 129,
    RemoveTrafficEndpoint = 130,
    TrafficEndpointId = 131,
    EthernetPacketFilter = 132,
    MacAddress = 133,
    CTag = 134,
    STag = 135,
    Ethertype = 136,
    Proxying = 137,
    EthernetFilterId = 138,
    EthernetFilterProperties = 139,
    SuggestedBufferingPacketsCount = 140,
    UserId = 141,
    EthernetPduSessionInformation = 142,
    EthernetTrafficInformation = 143,
    MacAddressesDetected = 144,
    MacAddressesRemoved = 145,
    EthernetInactivityTimer = 146,
    AdditionalMonitoringTime = 147,
    EventQuota = 148,
    EventThreshold = 149,
    SubsequentEventQuota = 150,
    SubsequentEventThreshold = 151,
    TraceInformation = 152,
    FramedRoute = 153,
    FramedRouting = 154,
    FramedIpv6Route = 155,
    TimeStamp = 156,
    AveragingWindow = 157,
    PagingPolicyIndicator = 158,
    ApnDnn = 159,
    ThreeGppInterfaceType = 160,
    PfcpSrreqFlags = 161,
    PfcpAureqFlags = 162,
    ActivationTime = 163,
    DeactivationTime = 164,
    CreateMar = 165,
    ThreeGppAccessForwardingActionInformation = 166,
    Non3gppAccessForwardingActionInformation = 167,
    RemoveMar = 168,
    UpdateMar = 169,
    MarId = 170,
    SteeringFunctionality = 171,
    SteeringMode = 172,
    Weight = 173,
    Priority = 174,
    Update3gppAccessForwardingActionInformation = 175,
    UpdateNon3gppAccessForwardingActionInformation = 176,
    UeIpAddressPoolIdentity = 177,
    AlternativeSmfIpAddress = 178,
    PacketReplicationAndDetectionCarryOnInformation = 179,
    SmfSetId = 180,
    QuotaValidityTime = 181,
    NumberOfReports = 182,
    PfcpSessionRetentionInformation = 183,
    PfcpAsrspFlags = 184,
    CpPfcpEntityIpAddress = 185,
    PfcpSereqFlags = 186,
    UserPlanePathRecoveryReport = 187,
    IpMulticastAddressingInfo = 188,
    JoinIpMulticastInformation = 189,
    LeaveIpMulticastInformation = 190,
    IpMulticastAddress = 191,
    SourceIpAddress = 192,
    PacketRateStatus = 193,
    CreateBridgeInfoForTsc = 194,
    CreatedBridgeInfoForTsc = 195,
    DsTtPortNumber = 196,
    NwTtPortNumber = 197,
    FivegsUserPlaneNode = 198,
    TscManagementInformationSmr = 199,
    TscManagementInformationSmrsp = 200,
    TscManagementInformationSrr = 201,
    PortManagementInformationContainer = 202,
    ClockDriftControlInformation = 203,
    RequestedClockDriftInformation = 204,
    ClockDriftReport = 205,
    TimeDomainNumber = 206,
    TimeOffsetThreshold = 207,
    CumulativeRateratioThreshold = 208,
    TimeOffsetMeasurement = 209,
    CumulativeRateratioMeasurement = 210,
    RemoveSrr = 211,
    CreateSrr = 212,
    UpdateSrr = 213,
    SessionReport = 214,
    SrrId = 215,
    AccessAvailabilityControlInformation = 216,
    RequestedAccessAvailabilityInformation = 217,
    AccessAvailabilityReport = 218,
    AccessAvailabilityInformation = 219,
    ProvideAtsssControlInformation = 220,
    AtsssControlParameters = 221,
    MptcpControlInformation = 222,
    AtsssLlControlInformation = 223,
    PmfControlInformation = 224,
    MptcpParameters = 225,
    AtsssLlParameters = 226,
    PmfParameters = 227,
    MptcpAddressInformation = 228,
    UeLinkSpecificIpAddress = 229,
    PmfAddressInformation = 230,
    AtsssLlInformation = 231,
    DataNetworkAccessIdentifier = 232,
    UeIpAddressPoolInformation = 233,
    AveragePacketDelay = 234,
    MinimumPacketDelay = 235,
    MaximumPacketDelay = 236,
    QosReportTrigger = 237,
    GtpUPathQosControlInformation = 238,
    GtpUPathQosReport = 239,
    QosInformationInGtpUPathQosReport = 240,
    GtpUPathInterfaceType = 241,
    QosMonitoringPerQosFlowControlInformation = 242,
    RequestedQosMonitoring = 243,
    ReportingFrequency = 244,
    PacketDelayThresholds = 245,
    MinimumWaitTime = 246,
    QosMonitoringReport = 247,
    QosMonitoringMeasurement = 248,
    MtEdtControlInformation = 249,
    DlDataPacketsSize = 250,
    QerControlIndications = 251,
    PacketRateStatusReport = 252,
    NfInstanceId = 253,
    EthernetContextInformation = 254,
    RedundantTransmissionParameters = 255,
    UpdatedPdr = 256,
    Snssai = 257,
    IpVersion = 258,
    PfcpAsreqFlags = 259,
    DataStatus = 260,
    ProvideRdsConfigurationInformation = 261,
    RdsConfigurationInformation = 262,
    QueryPacketRateStatus = 263,
    PacketRateStatusReportWithinSessionModificationResponse = 264,
    MultipathApplicableIndication = 265,
    UserPlaneNodeManagementInformationContainer = 266,
    UeIpAddressUsageInformation = 267,
    NumberOfUeIpAddresses = 268,
    ValidityTimer = 269,
    RedundantTransmissionForwardingParameters = 270,
    TransportDelayReporting = 271,
    PartialFailureInformation = 272,
    Reserved273 = 273,
    OffendingIeInformation = 274,
    RatType = 275,
    L2tpTunnelInformation = 276,
    L2tpSessionInformation = 277,
    L2tpUserAuthentication = 278,
    CreatedL2tpSession = 279,
    LnsAddress = 280,
    TunnelPreference = 281,
    CallingNumber = 282,
    CalledNumber = 283,
    L2tpSessionIndications = 284,
    DnsServerAddress = 285,
    NbnsServerAddress = 286,
    MaximumReceiveUnit = 287,
    Thresholds = 288,
    SteeringModeIndicator = 289,
    PfcpSessionChangeInfo = 290,
    GroupId = 291,
    CpIpAddress = 292,
    IpAddressAndPortNumberReplacement = 293,
    DnsQueryResponseFilter = 294,
    DirectReportingInformation = 295,
    EventNotificationUri = 296,
    NotificationCorrelationId = 297,
    ReportingFlags = 298,
    PredefinedRulesName = 299,
    MbsSessionN4mbControlInformation = 300,
    MbsMulticastParameters = 301,
    AddMbsUnicastParameters = 302,
    MbsSessionN4mbInformation = 303,
    RemoveMbsUnicastParameters = 304,
    MbsSessionIdentifier = 305,
    MulticastTransportInformation = 306,
    Mbsn4mbReqFlags = 307,
    LocalIngressTunnel = 308,
    MbsUnicastParametersId = 309,
    MbsSessionN4ControlInformation = 310,
    MbsSessionN4Information = 311,
    Mbsn4RespFlags = 312,
    TunnelPassword = 313,
    AreaSessionId = 314,
    PeerUpRestartReport = 315,
    DscpToPpiControlInformation = 316,
    DscpToPpiMappingInformation = 317,
    PfcpsdrspFlags = 318,
    QerIndications = 319,
    VendorSpecificNodeReportType = 320,
    ConfiguredTimeDomain = 321,
    Metadata = 322,
    TrafficParameterMeasurementControlInformation = 323,
    TrafficParameterMeasurementReport = 324,
    TrafficParameterThreshold = 325,
    DlPeriodicity = 326,
    N6JitterMeasurement = 327,
    TrafficParameterMeasurementIndication = 328,
    UlPeriodicity = 329,
    MpquicControlInformation = 330,
    MpquicParameters = 331,
    MpquicAddressInformation = 332,
    TransportMode = 333,
    ProtocolDescription = 334,
    ReportingSuggestionInfo = 335,
    TlContainer = 336,
    MeasurementIndication = 337,
    HplmnSnssai = 338,
    MediaTransportProtocol = 339,
    RtpHeaderExtensionInformation = 340,
    RtpPayloadInformation = 341,
    RtpHeaderExtensionType = 342,
    RtpHeaderExtensionId = 343,
    RtpPayloadType = 344,
    RtpPayloadFormat = 345,
    ExtendedDlBufferingNotificationPolicy = 346,
    MtSdtControlInformation = 347,
    ReportingThresholds = 348,
    RtpHeaderExtensionAdditionalInformation = 349,
    MappedN6IpAddress = 350,
    N6RoutingInformation = 351,
    Uri = 352,
    UeLevelMeasurementsConfiguration = 353,
    N6DelayMeasurementProtocols = 354,
    N6DelayMeasurementControlInformation = 355,
    N6DelayMeasurementReport = 356,
    N6DelayMeasurementInformation = 357,
    MeasurementEndpointAddress = 358,
    OperatorConfigurableUpfCapability = 359,
    PacketInspectionFunctionality = 360,
    HeaderHandlingControlRule = 361,
    HeaderHandlingReportingControlInfo = 362,
    HeaderHandlingControlInformation = 363,
    HeaderDetectionReference = 364,
    HeaderDetectionSupportInformation = 365,
    ReportingEndpointId = 366,
    HeaderHandlingControlReference = 367,
    HeaderHandlingAction = 368,
    HeaderInformation = 369,
    HeaderValue = 370,
    HeaderHandlingCondition = 371,
    HeaderHandlingControlId = 372,
    HeaderHandlingControlRuleId = 373,
    OnPathN6ConnectionInformation = 374,
    MeasurementReportingType = 375,
    N6DelayMeasurementFailureInformation = 376,
    N6DelayMeasurementControlInformationId = 377,
    ProtocolSpecificConfigurationParameters = 378,
    MeasurementEndpointPortNumber = 379,
    HeaderHandlingReportingIndication = 380,
    Reserved381 = 381,
    SmfChangeReason = 382,
    ExtendedTransportLevelMarking = 383,
    PduSetImportance = 384,
    MoqControlInformation = 385,
    MoqInformation = 386,
    MoqRelayIpAddress = 387,
    MediaRelatedInformationTransferInfo = 388,
    ReportingControlInformation = 389,
    SecurityModeStamp = 390,
    HmacKeyStamp = 391,
    SecurityModeOwampTwamp = 392,
    KeyIdAndSharedSecretOwampTwamp = 393,
    RemainingDataReportingIndication = 394,
    ExpeditedTransferIndication = 395,
    SessionReflectorModeStamp = 396,
    PfdPartialFailureInformation = 397,
    TransportLevelMarkingIndications = 398,
    RedundantN3N9TransmissionInformation = 399,
    LocalN3N9TunnelInformation = 400,
    RemoteN3N9TunnelInformation = 401,
    BindingIndication = 402,
    PduSetImportanceForN6UnmarkedPdUs = 403,
}

// NOTE: dispatch throughout the crate compares raw `IeType::X as u16` values, so
// no `TryFrom<u16>` conversion is needed. A partial table was removed (pfcp-11)
// to avoid a latent inconsistency where only a subset of discriminants mapped.

/// PFCP IE Header (4 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IeHeader {
    pub ie_type: u16,
    pub length: u16,
}

impl IeHeader {
    pub const LEN: usize = 4;

    pub fn new(ie_type: u16, length: u16) -> Self {
        Self { ie_type, length }
    }

    pub fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.ie_type);
        buf.put_u16(self.length);
    }

    pub fn decode(buf: &mut Bytes) -> PfcpResult<Self> {
        if buf.remaining() < Self::LEN {
            return Err(PfcpError::BufferTooShort {
                needed: Self::LEN,
                available: buf.remaining(),
            });
        }
        Ok(Self {
            ie_type: buf.get_u16(),
            length: buf.get_u16(),
        })
    }
}

/// Generic PFCP IE with raw data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawIe {
    pub ie_type: u16,
    pub data: Bytes,
}

impl RawIe {
    pub fn new(ie_type: u16, data: Bytes) -> Self {
        Self { ie_type, data }
    }

    pub fn encode(&self, buf: &mut BytesMut) {
        let header = IeHeader::new(self.ie_type, self.data.len() as u16);
        header.encode(buf);
        buf.put_slice(&self.data);
    }

    pub fn decode(buf: &mut Bytes) -> PfcpResult<Self> {
        let header = IeHeader::decode(buf)?;
        if buf.remaining() < header.length as usize {
            return Err(PfcpError::BufferTooShort {
                needed: header.length as usize,
                available: buf.remaining(),
            });
        }
        let data = buf.copy_to_bytes(header.length as usize);
        Ok(Self {
            ie_type: header.ie_type,
            data,
        })
    }
}

/// Helper to encode a u8 IE
pub fn encode_u8_ie(buf: &mut BytesMut, ie_type: IeType, value: u8) {
    let header = IeHeader::new(ie_type as u16, 1);
    header.encode(buf);
    buf.put_u8(value);
}

/// Helper to encode a u16 IE
pub fn encode_u16_ie(buf: &mut BytesMut, ie_type: IeType, value: u16) {
    let header = IeHeader::new(ie_type as u16, 2);
    header.encode(buf);
    buf.put_u16(value);
}

/// Helper to encode a u32 IE
pub fn encode_u32_ie(buf: &mut BytesMut, ie_type: IeType, value: u32) {
    let header = IeHeader::new(ie_type as u16, 4);
    header.encode(buf);
    buf.put_u32(value);
}

/// Helper to encode a u64 IE
pub fn encode_u64_ie(buf: &mut BytesMut, ie_type: IeType, value: u64) {
    let header = IeHeader::new(ie_type as u16, 8);
    header.encode(buf);
    buf.put_u64(value);
}

/// Helper to encode bytes IE
pub fn encode_bytes_ie(buf: &mut BytesMut, ie_type: IeType, data: &[u8]) {
    let header = IeHeader::new(ie_type as u16, data.len() as u16);
    header.encode(buf);
    buf.put_slice(data);
}

// ============================================================================
// Structured PFCP IE Encoders/Decoders
// ============================================================================

/// DurationMeasurement (IE 67) - Duration in seconds
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationMeasurement(pub u32);

impl DurationMeasurement {
    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u32(self.0);
        buf.freeze()
    }

    pub fn decode(data: &[u8]) -> PfcpResult<Self> {
        if data.len() < 4 {
            return Err(PfcpError::BufferTooShort {
                needed: 4,
                available: data.len(),
            });
        }
        let mut buf = Bytes::copy_from_slice(data);
        Ok(DurationMeasurement(buf.get_u32()))
    }
}

// NOTE: VolumeThreshold (IE 31) and ReportType (IE 39) had duplicate, divergent
// definitions here that no consumer referenced; the canonical copies live in
// `crate::types` (used by message.rs / prelude). The dead copies were removed
// (pfcp-12).

/// UsageReportTrigger (IE 63) - Trigger for usage report
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportTrigger {
    pub immediate_report: bool,
    pub volume_threshold: bool,
    pub time_threshold: bool,
    pub periodic_reporting: bool,
    pub event_threshold: bool,
}

impl UsageReportTrigger {
    pub fn encode(&self) -> Bytes {
        let mut flags = 0u16;
        if self.immediate_report {
            flags |= 0x0001;
        }
        if self.volume_threshold {
            flags |= 0x0002;
        }
        if self.time_threshold {
            flags |= 0x0004;
        }
        if self.periodic_reporting {
            flags |= 0x0008;
        }
        if self.event_threshold {
            flags |= 0x0010;
        }

        let mut buf = BytesMut::new();
        buf.put_u16(flags);
        buf.freeze()
    }

    pub fn decode(data: &[u8]) -> PfcpResult<Self> {
        if data.len() < 2 {
            return Err(PfcpError::BufferTooShort {
                needed: 2,
                available: data.len(),
            });
        }
        let mut buf = Bytes::copy_from_slice(data);
        let flags = buf.get_u16();

        Ok(UsageReportTrigger {
            immediate_report: flags & 0x0001 != 0,
            volume_threshold: flags & 0x0002 != 0,
            time_threshold: flags & 0x0004 != 0,
            periodic_reporting: flags & 0x0008 != 0,
            event_threshold: flags & 0x0010 != 0,
        })
    }
}

/// PacketRate (IE 94) - UL/DL packet rate limits
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PacketRate {
    pub uplink_time_unit: Option<u8>,
    pub max_uplink_packet_rate: Option<u16>,
    pub downlink_time_unit: Option<u8>,
    pub max_downlink_packet_rate: Option<u16>,
}

impl PacketRate {
    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();
        let mut flags = 0u8;
        if self.uplink_time_unit.is_some() {
            flags |= 0x01;
        }
        if self.downlink_time_unit.is_some() {
            flags |= 0x02;
        }

        buf.put_u8(flags);
        if let Some(unit) = self.uplink_time_unit {
            buf.put_u8(unit);
            buf.put_u16(self.max_uplink_packet_rate.unwrap_or(0));
        }
        if let Some(unit) = self.downlink_time_unit {
            buf.put_u8(unit);
            buf.put_u16(self.max_downlink_packet_rate.unwrap_or(0));
        }

        buf.freeze()
    }

    pub fn decode(data: &[u8]) -> PfcpResult<Self> {
        if data.is_empty() {
            return Err(PfcpError::BufferTooShort {
                needed: 1,
                available: 0,
            });
        }
        let mut buf = Bytes::copy_from_slice(data);
        let flags = buf.get_u8();
        let mut pr = PacketRate::default();

        if flags & 0x01 != 0 {
            pr.uplink_time_unit = Some(buf.get_u8());
            pr.max_uplink_packet_rate = Some(buf.get_u16());
        }
        if flags & 0x02 != 0 {
            pr.downlink_time_unit = Some(buf.get_u8());
            pr.max_downlink_packet_rate = Some(buf.get_u16());
        }

        Ok(pr)
    }
}

/// QerControlIndications (IE 251) - QER control indications
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QerControlIndications {
    pub rcsr: bool, // Rate Control Status Reporting
}

impl QerControlIndications {
    pub fn encode(&self) -> Bytes {
        let flags = if self.rcsr { 0x01 } else { 0x00 };
        Bytes::copy_from_slice(&[flags])
    }

    pub fn decode(data: &[u8]) -> PfcpResult<Self> {
        if data.is_empty() {
            return Err(PfcpError::BufferTooShort {
                needed: 1,
                available: 0,
            });
        }
        Ok(QerControlIndications {
            rcsr: data[0] & 0x01 != 0,
        })
    }
}

// NOTE: MeasurementMethod (IE 62) had a duplicate, divergent definition here
// that no consumer referenced; the canonical copy lives in `crate::types`
// (used by message.rs / prelude). The dead copy was removed (pfcp-12).

/// MeasurementInformation (IE 100) - Information about measurements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementInformation {
    pub mbqe: bool, // Measurement Before QoS Enforcement
    pub inam: bool, // Inactive Measurement
    pub radi: bool, // Reduced Application Detection Information
    pub istm: bool, // Immediate Start Time Metering
}

impl MeasurementInformation {
    pub fn encode(&self) -> Bytes {
        let mut flags = 0u8;
        if self.mbqe {
            flags |= 0x01;
        }
        if self.inam {
            flags |= 0x02;
        }
        if self.radi {
            flags |= 0x04;
        }
        if self.istm {
            flags |= 0x08;
        }
        Bytes::copy_from_slice(&[flags])
    }

    pub fn decode(data: &[u8]) -> PfcpResult<Self> {
        if data.is_empty() {
            return Err(PfcpError::BufferTooShort {
                needed: 1,
                available: 0,
            });
        }
        Ok(MeasurementInformation {
            mbqe: data[0] & 0x01 != 0,
            inam: data[0] & 0x02 != 0,
            radi: data[0] & 0x04 != 0,
            istm: data[0] & 0x08 != 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ie_header_encode_decode() {
        let header = IeHeader::new(IeType::Cause as u16, 1);
        let mut buf = BytesMut::new();
        header.encode(&mut buf);

        let mut bytes = buf.freeze();
        let decoded = IeHeader::decode(&mut bytes).unwrap();

        assert_eq!(decoded.ie_type, IeType::Cause as u16);
        assert_eq!(decoded.length, 1);
    }

    #[test]
    fn test_raw_ie_encode_decode() {
        let ie = RawIe::new(IeType::Cause as u16, Bytes::from_static(&[0x01]));
        let mut buf = BytesMut::new();
        ie.encode(&mut buf);

        let mut bytes = buf.freeze();
        let decoded = RawIe::decode(&mut bytes).unwrap();

        assert_eq!(decoded.ie_type, IeType::Cause as u16);
        assert_eq!(decoded.data.as_ref(), &[0x01]);
    }

    #[test]
    fn test_pfcp_ie_type_values_match_spec() {
        assert_eq!(IeType::Reserved0 as u16, 0);
        assert_eq!(IeType::CreatePdr as u16, 1);
        assert_eq!(IeType::Pdi as u16, 2);
        assert_eq!(IeType::CreateFar as u16, 3);
        assert_eq!(IeType::ForwardingParameters as u16, 4);
        assert_eq!(IeType::DuplicatingParameters as u16, 5);
        assert_eq!(IeType::CreateUrr as u16, 6);
        assert_eq!(IeType::CreateQer as u16, 7);
        assert_eq!(IeType::CreatedPdr as u16, 8);
        assert_eq!(IeType::UpdatePdr as u16, 9);
        assert_eq!(IeType::UpdateFar as u16, 10);
        assert_eq!(IeType::UpdateForwardingParameters as u16, 11);
        assert_eq!(IeType::UpdateBar as u16, 12);
        assert_eq!(IeType::UpdateUrr as u16, 13);
        assert_eq!(IeType::UpdateQer as u16, 14);
        assert_eq!(IeType::RemovePdr as u16, 15);
        assert_eq!(IeType::RemoveFar as u16, 16);
        assert_eq!(IeType::RemoveUrr as u16, 17);
        assert_eq!(IeType::RemoveQer as u16, 18);
        assert_eq!(IeType::Cause as u16, 19);
        assert_eq!(IeType::SourceInterface as u16, 20);
        assert_eq!(IeType::FTeid as u16, 21);
        assert_eq!(IeType::NetworkInstance as u16, 22);
        assert_eq!(IeType::SdfFilter as u16, 23);
        assert_eq!(IeType::ApplicationId as u16, 24);
        assert_eq!(IeType::GateStatus as u16, 25);
        assert_eq!(IeType::Mbr as u16, 26);
        assert_eq!(IeType::Gbr as u16, 27);
        assert_eq!(IeType::QerCorrelationId as u16, 28);
        assert_eq!(IeType::Precedence as u16, 29);
        assert_eq!(IeType::TransportLevelMarking as u16, 30);
        assert_eq!(IeType::VolumeThreshold as u16, 31);
        assert_eq!(IeType::TimeThreshold as u16, 32);
        assert_eq!(IeType::MonitoringTime as u16, 33);
        assert_eq!(IeType::SubsequentVolumeThreshold as u16, 34);
        assert_eq!(IeType::SubsequentTimeThreshold as u16, 35);
        assert_eq!(IeType::InactivityDetectionTime as u16, 36);
        assert_eq!(IeType::ReportingTriggers as u16, 37);
        assert_eq!(IeType::RedirectInformation as u16, 38);
        assert_eq!(IeType::ReportType as u16, 39);
        assert_eq!(IeType::OffendingIe as u16, 40);
        assert_eq!(IeType::ForwardingPolicy as u16, 41);
        assert_eq!(IeType::DestinationInterface as u16, 42);
        assert_eq!(IeType::UpFunctionFeatures as u16, 43);
        assert_eq!(IeType::ApplyAction as u16, 44);
        assert_eq!(IeType::DownlinkDataServiceInformation as u16, 45);
        assert_eq!(IeType::DownlinkDataNotificationDelay as u16, 46);
        assert_eq!(IeType::DlBufferingDuration as u16, 47);
        assert_eq!(IeType::DlBufferingSuggestedPacketCount as u16, 48);
        assert_eq!(IeType::PfcpSmreqFlags as u16, 49);
        assert_eq!(IeType::PfcpSrrspFlags as u16, 50);
        assert_eq!(IeType::LoadControlInformation as u16, 51);
        assert_eq!(IeType::SequenceNumber as u16, 52);
        assert_eq!(IeType::Metric as u16, 53);
        assert_eq!(IeType::OverloadControlInformation as u16, 54);
        assert_eq!(IeType::Timer as u16, 55);
        assert_eq!(IeType::PdrId as u16, 56);
        assert_eq!(IeType::FSeid as u16, 57);
        assert_eq!(IeType::ApplicationIdsPfds as u16, 58);
        assert_eq!(IeType::PfdContext as u16, 59);
        assert_eq!(IeType::NodeId as u16, 60);
        assert_eq!(IeType::PfdContents as u16, 61);
        assert_eq!(IeType::MeasurementMethod as u16, 62);
        assert_eq!(IeType::UsageReportTrigger as u16, 63);
        assert_eq!(IeType::MeasurementPeriod as u16, 64);
        assert_eq!(IeType::FqCsid as u16, 65);
        assert_eq!(IeType::VolumeMeasurement as u16, 66);
        assert_eq!(IeType::DurationMeasurement as u16, 67);
        assert_eq!(IeType::ApplicationDetectionInformation as u16, 68);
        assert_eq!(IeType::TimeOfFirstPacket as u16, 69);
        assert_eq!(IeType::TimeOfLastPacket as u16, 70);
        assert_eq!(IeType::QuotaHoldingTime as u16, 71);
        assert_eq!(IeType::DroppedDlTrafficThreshold as u16, 72);
        assert_eq!(IeType::VolumeQuota as u16, 73);
        assert_eq!(IeType::TimeQuota as u16, 74);
        assert_eq!(IeType::StartTime as u16, 75);
        assert_eq!(IeType::EndTime as u16, 76);
        assert_eq!(IeType::QueryUrr as u16, 77);
        assert_eq!(IeType::UsageReportSmr as u16, 78);
        assert_eq!(IeType::UsageReportSdr as u16, 79);
        assert_eq!(IeType::UsageReportSrr as u16, 80);
        assert_eq!(IeType::UrrId as u16, 81);
        assert_eq!(IeType::LinkedUrrId as u16, 82);
        assert_eq!(IeType::DownlinkDataReport as u16, 83);
        assert_eq!(IeType::OuterHeaderCreation as u16, 84);
        assert_eq!(IeType::CreateBar as u16, 85);
        assert_eq!(IeType::UpdateBarSmr as u16, 86);
        assert_eq!(IeType::RemoveBar as u16, 87);
        assert_eq!(IeType::BarId as u16, 88);
        assert_eq!(IeType::CpFunctionFeatures as u16, 89);
        assert_eq!(IeType::UsageInformation as u16, 90);
        assert_eq!(IeType::ApplicationInstanceId as u16, 91);
        assert_eq!(IeType::FlowInformation as u16, 92);
        assert_eq!(IeType::UeIpAddress as u16, 93);
        assert_eq!(IeType::PacketRate as u16, 94);
        assert_eq!(IeType::OuterHeaderRemoval as u16, 95);
        assert_eq!(IeType::RecoveryTimeStamp as u16, 96);
        assert_eq!(IeType::DlFlowLevelMarking as u16, 97);
        assert_eq!(IeType::HeaderEnrichment as u16, 98);
        assert_eq!(IeType::ErrorIndicationReport as u16, 99);
        assert_eq!(IeType::MeasurementInformation as u16, 100);
        assert_eq!(IeType::NodeReportType as u16, 101);
        assert_eq!(IeType::UserPlanePathFailureReport as u16, 102);
        assert_eq!(IeType::RemoteGtpUPeer as u16, 103);
        assert_eq!(IeType::UrSeqn as u16, 104);
        assert_eq!(IeType::UpdateDuplicatingParameters as u16, 105);
        assert_eq!(IeType::ActivatePredefinedRules as u16, 106);
        assert_eq!(IeType::DeactivatePredefinedRules as u16, 107);
        assert_eq!(IeType::FarId as u16, 108);
        assert_eq!(IeType::QerId as u16, 109);
        assert_eq!(IeType::OciFlags as u16, 110);
        assert_eq!(IeType::PfcpAssociationReleaseRequest as u16, 111);
        assert_eq!(IeType::GracefulReleasePeriod as u16, 112);
        assert_eq!(IeType::PdnType as u16, 113);
        assert_eq!(IeType::FailedRuleId as u16, 114);
        assert_eq!(IeType::TimeQuotaMechanism as u16, 115);
        assert_eq!(IeType::Reserved116 as u16, 116);
        assert_eq!(IeType::UserPlaneInactivityTimer as u16, 117);
        assert_eq!(IeType::AggregatedUrrs as u16, 118);
        assert_eq!(IeType::Multiplier as u16, 119);
        assert_eq!(IeType::AggregatedUrrId as u16, 120);
        assert_eq!(IeType::SubsequentVolumeQuota as u16, 121);
        assert_eq!(IeType::SubsequentTimeQuota as u16, 122);
        assert_eq!(IeType::Rqi as u16, 123);
        assert_eq!(IeType::Qfi as u16, 124);
        assert_eq!(IeType::QueryUrrReference as u16, 125);
        assert_eq!(IeType::AdditionalUsageReportsInformation as u16, 126);
        assert_eq!(IeType::CreateTrafficEndpoint as u16, 127);
        assert_eq!(IeType::CreatedTrafficEndpoint as u16, 128);
        assert_eq!(IeType::UpdateTrafficEndpoint as u16, 129);
        assert_eq!(IeType::RemoveTrafficEndpoint as u16, 130);
        assert_eq!(IeType::TrafficEndpointId as u16, 131);
        assert_eq!(IeType::EthernetPacketFilter as u16, 132);
        assert_eq!(IeType::MacAddress as u16, 133);
        assert_eq!(IeType::CTag as u16, 134);
        assert_eq!(IeType::STag as u16, 135);
        assert_eq!(IeType::Ethertype as u16, 136);
        assert_eq!(IeType::Proxying as u16, 137);
        assert_eq!(IeType::EthernetFilterId as u16, 138);
        assert_eq!(IeType::EthernetFilterProperties as u16, 139);
        assert_eq!(IeType::SuggestedBufferingPacketsCount as u16, 140);
        assert_eq!(IeType::UserId as u16, 141);
        assert_eq!(IeType::EthernetPduSessionInformation as u16, 142);
        assert_eq!(IeType::EthernetTrafficInformation as u16, 143);
        assert_eq!(IeType::MacAddressesDetected as u16, 144);
        assert_eq!(IeType::MacAddressesRemoved as u16, 145);
        assert_eq!(IeType::EthernetInactivityTimer as u16, 146);
        assert_eq!(IeType::AdditionalMonitoringTime as u16, 147);
        assert_eq!(IeType::EventQuota as u16, 148);
        assert_eq!(IeType::EventThreshold as u16, 149);
        assert_eq!(IeType::SubsequentEventQuota as u16, 150);
        assert_eq!(IeType::SubsequentEventThreshold as u16, 151);
        assert_eq!(IeType::TraceInformation as u16, 152);
        assert_eq!(IeType::FramedRoute as u16, 153);
        assert_eq!(IeType::FramedRouting as u16, 154);
        assert_eq!(IeType::FramedIpv6Route as u16, 155);
        assert_eq!(IeType::TimeStamp as u16, 156);
        assert_eq!(IeType::AveragingWindow as u16, 157);
        assert_eq!(IeType::PagingPolicyIndicator as u16, 158);
        assert_eq!(IeType::ApnDnn as u16, 159);
        assert_eq!(IeType::ThreeGppInterfaceType as u16, 160);
        assert_eq!(IeType::PfcpSrreqFlags as u16, 161);
        assert_eq!(IeType::PfcpAureqFlags as u16, 162);
        assert_eq!(IeType::ActivationTime as u16, 163);
        assert_eq!(IeType::DeactivationTime as u16, 164);
        assert_eq!(IeType::CreateMar as u16, 165);
        assert_eq!(IeType::ThreeGppAccessForwardingActionInformation as u16, 166);
        assert_eq!(IeType::Non3gppAccessForwardingActionInformation as u16, 167);
        assert_eq!(IeType::RemoveMar as u16, 168);
        assert_eq!(IeType::UpdateMar as u16, 169);
        assert_eq!(IeType::MarId as u16, 170);
        assert_eq!(IeType::SteeringFunctionality as u16, 171);
        assert_eq!(IeType::SteeringMode as u16, 172);
        assert_eq!(IeType::Weight as u16, 173);
        assert_eq!(IeType::Priority as u16, 174);
        assert_eq!(IeType::Update3gppAccessForwardingActionInformation as u16, 175);
        assert_eq!(IeType::UpdateNon3gppAccessForwardingActionInformation as u16, 176);
        assert_eq!(IeType::UeIpAddressPoolIdentity as u16, 177);
        assert_eq!(IeType::AlternativeSmfIpAddress as u16, 178);
        assert_eq!(IeType::PacketReplicationAndDetectionCarryOnInformation as u16, 179);
        assert_eq!(IeType::SmfSetId as u16, 180);
        assert_eq!(IeType::QuotaValidityTime as u16, 181);
        assert_eq!(IeType::NumberOfReports as u16, 182);
        assert_eq!(IeType::PfcpSessionRetentionInformation as u16, 183);
        assert_eq!(IeType::PfcpAsrspFlags as u16, 184);
        assert_eq!(IeType::CpPfcpEntityIpAddress as u16, 185);
        assert_eq!(IeType::PfcpSereqFlags as u16, 186);
        assert_eq!(IeType::UserPlanePathRecoveryReport as u16, 187);
        assert_eq!(IeType::IpMulticastAddressingInfo as u16, 188);
        assert_eq!(IeType::JoinIpMulticastInformation as u16, 189);
        assert_eq!(IeType::LeaveIpMulticastInformation as u16, 190);
        assert_eq!(IeType::IpMulticastAddress as u16, 191);
        assert_eq!(IeType::SourceIpAddress as u16, 192);
        assert_eq!(IeType::PacketRateStatus as u16, 193);
        assert_eq!(IeType::CreateBridgeInfoForTsc as u16, 194);
        assert_eq!(IeType::CreatedBridgeInfoForTsc as u16, 195);
        assert_eq!(IeType::DsTtPortNumber as u16, 196);
        assert_eq!(IeType::NwTtPortNumber as u16, 197);
        assert_eq!(IeType::FivegsUserPlaneNode as u16, 198);
        assert_eq!(IeType::TscManagementInformationSmr as u16, 199);
        assert_eq!(IeType::TscManagementInformationSmrsp as u16, 200);
        assert_eq!(IeType::TscManagementInformationSrr as u16, 201);
        assert_eq!(IeType::PortManagementInformationContainer as u16, 202);
        assert_eq!(IeType::ClockDriftControlInformation as u16, 203);
        assert_eq!(IeType::RequestedClockDriftInformation as u16, 204);
        assert_eq!(IeType::ClockDriftReport as u16, 205);
        assert_eq!(IeType::TimeDomainNumber as u16, 206);
        assert_eq!(IeType::TimeOffsetThreshold as u16, 207);
        assert_eq!(IeType::CumulativeRateratioThreshold as u16, 208);
        assert_eq!(IeType::TimeOffsetMeasurement as u16, 209);
        assert_eq!(IeType::CumulativeRateratioMeasurement as u16, 210);
        assert_eq!(IeType::RemoveSrr as u16, 211);
        assert_eq!(IeType::CreateSrr as u16, 212);
        assert_eq!(IeType::UpdateSrr as u16, 213);
        assert_eq!(IeType::SessionReport as u16, 214);
        assert_eq!(IeType::SrrId as u16, 215);
        assert_eq!(IeType::AccessAvailabilityControlInformation as u16, 216);
        assert_eq!(IeType::RequestedAccessAvailabilityInformation as u16, 217);
        assert_eq!(IeType::AccessAvailabilityReport as u16, 218);
        assert_eq!(IeType::AccessAvailabilityInformation as u16, 219);
        assert_eq!(IeType::ProvideAtsssControlInformation as u16, 220);
        assert_eq!(IeType::AtsssControlParameters as u16, 221);
        assert_eq!(IeType::MptcpControlInformation as u16, 222);
        assert_eq!(IeType::AtsssLlControlInformation as u16, 223);
        assert_eq!(IeType::PmfControlInformation as u16, 224);
        assert_eq!(IeType::MptcpParameters as u16, 225);
        assert_eq!(IeType::AtsssLlParameters as u16, 226);
        assert_eq!(IeType::PmfParameters as u16, 227);
        assert_eq!(IeType::MptcpAddressInformation as u16, 228);
        assert_eq!(IeType::UeLinkSpecificIpAddress as u16, 229);
        assert_eq!(IeType::PmfAddressInformation as u16, 230);
        assert_eq!(IeType::AtsssLlInformation as u16, 231);
        assert_eq!(IeType::DataNetworkAccessIdentifier as u16, 232);
        assert_eq!(IeType::UeIpAddressPoolInformation as u16, 233);
        assert_eq!(IeType::AveragePacketDelay as u16, 234);
        assert_eq!(IeType::MinimumPacketDelay as u16, 235);
        assert_eq!(IeType::MaximumPacketDelay as u16, 236);
        assert_eq!(IeType::QosReportTrigger as u16, 237);
        assert_eq!(IeType::GtpUPathQosControlInformation as u16, 238);
        assert_eq!(IeType::GtpUPathQosReport as u16, 239);
        assert_eq!(IeType::QosInformationInGtpUPathQosReport as u16, 240);
        assert_eq!(IeType::GtpUPathInterfaceType as u16, 241);
        assert_eq!(IeType::QosMonitoringPerQosFlowControlInformation as u16, 242);
        assert_eq!(IeType::RequestedQosMonitoring as u16, 243);
        assert_eq!(IeType::ReportingFrequency as u16, 244);
        assert_eq!(IeType::PacketDelayThresholds as u16, 245);
        assert_eq!(IeType::MinimumWaitTime as u16, 246);
        assert_eq!(IeType::QosMonitoringReport as u16, 247);
        assert_eq!(IeType::QosMonitoringMeasurement as u16, 248);
        assert_eq!(IeType::MtEdtControlInformation as u16, 249);
        assert_eq!(IeType::DlDataPacketsSize as u16, 250);
        assert_eq!(IeType::QerControlIndications as u16, 251);
        assert_eq!(IeType::PacketRateStatusReport as u16, 252);
        assert_eq!(IeType::NfInstanceId as u16, 253);
        assert_eq!(IeType::EthernetContextInformation as u16, 254);
        assert_eq!(IeType::RedundantTransmissionParameters as u16, 255);
        assert_eq!(IeType::UpdatedPdr as u16, 256);
        assert_eq!(IeType::Snssai as u16, 257);
        assert_eq!(IeType::IpVersion as u16, 258);
        assert_eq!(IeType::PfcpAsreqFlags as u16, 259);
        assert_eq!(IeType::DataStatus as u16, 260);
        assert_eq!(IeType::ProvideRdsConfigurationInformation as u16, 261);
        assert_eq!(IeType::RdsConfigurationInformation as u16, 262);
        assert_eq!(IeType::QueryPacketRateStatus as u16, 263);
        assert_eq!(IeType::PacketRateStatusReportWithinSessionModificationResponse as u16, 264);
        assert_eq!(IeType::MultipathApplicableIndication as u16, 265);
        assert_eq!(IeType::UserPlaneNodeManagementInformationContainer as u16, 266);
        assert_eq!(IeType::UeIpAddressUsageInformation as u16, 267);
        assert_eq!(IeType::NumberOfUeIpAddresses as u16, 268);
        assert_eq!(IeType::ValidityTimer as u16, 269);
        assert_eq!(IeType::RedundantTransmissionForwardingParameters as u16, 270);
        assert_eq!(IeType::TransportDelayReporting as u16, 271);
        assert_eq!(IeType::PartialFailureInformation as u16, 272);
        assert_eq!(IeType::Reserved273 as u16, 273);
        assert_eq!(IeType::OffendingIeInformation as u16, 274);
        assert_eq!(IeType::RatType as u16, 275);
        assert_eq!(IeType::L2tpTunnelInformation as u16, 276);
        assert_eq!(IeType::L2tpSessionInformation as u16, 277);
        assert_eq!(IeType::L2tpUserAuthentication as u16, 278);
        assert_eq!(IeType::CreatedL2tpSession as u16, 279);
        assert_eq!(IeType::LnsAddress as u16, 280);
        assert_eq!(IeType::TunnelPreference as u16, 281);
        assert_eq!(IeType::CallingNumber as u16, 282);
        assert_eq!(IeType::CalledNumber as u16, 283);
        assert_eq!(IeType::L2tpSessionIndications as u16, 284);
        assert_eq!(IeType::DnsServerAddress as u16, 285);
        assert_eq!(IeType::NbnsServerAddress as u16, 286);
        assert_eq!(IeType::MaximumReceiveUnit as u16, 287);
        assert_eq!(IeType::Thresholds as u16, 288);
        assert_eq!(IeType::SteeringModeIndicator as u16, 289);
        assert_eq!(IeType::PfcpSessionChangeInfo as u16, 290);
        assert_eq!(IeType::GroupId as u16, 291);
        assert_eq!(IeType::CpIpAddress as u16, 292);
        assert_eq!(IeType::IpAddressAndPortNumberReplacement as u16, 293);
        assert_eq!(IeType::DnsQueryResponseFilter as u16, 294);
        assert_eq!(IeType::DirectReportingInformation as u16, 295);
        assert_eq!(IeType::EventNotificationUri as u16, 296);
        assert_eq!(IeType::NotificationCorrelationId as u16, 297);
        assert_eq!(IeType::ReportingFlags as u16, 298);
        assert_eq!(IeType::PredefinedRulesName as u16, 299);
        assert_eq!(IeType::MbsSessionN4mbControlInformation as u16, 300);
        assert_eq!(IeType::MbsMulticastParameters as u16, 301);
        assert_eq!(IeType::AddMbsUnicastParameters as u16, 302);
        assert_eq!(IeType::MbsSessionN4mbInformation as u16, 303);
        assert_eq!(IeType::RemoveMbsUnicastParameters as u16, 304);
        assert_eq!(IeType::MbsSessionIdentifier as u16, 305);
        assert_eq!(IeType::MulticastTransportInformation as u16, 306);
        assert_eq!(IeType::Mbsn4mbReqFlags as u16, 307);
        assert_eq!(IeType::LocalIngressTunnel as u16, 308);
        assert_eq!(IeType::MbsUnicastParametersId as u16, 309);
        assert_eq!(IeType::MbsSessionN4ControlInformation as u16, 310);
        assert_eq!(IeType::MbsSessionN4Information as u16, 311);
        assert_eq!(IeType::Mbsn4RespFlags as u16, 312);
        assert_eq!(IeType::TunnelPassword as u16, 313);
        assert_eq!(IeType::AreaSessionId as u16, 314);
        assert_eq!(IeType::PeerUpRestartReport as u16, 315);
        assert_eq!(IeType::DscpToPpiControlInformation as u16, 316);
        assert_eq!(IeType::DscpToPpiMappingInformation as u16, 317);
        assert_eq!(IeType::PfcpsdrspFlags as u16, 318);
        assert_eq!(IeType::QerIndications as u16, 319);
        assert_eq!(IeType::VendorSpecificNodeReportType as u16, 320);
        assert_eq!(IeType::ConfiguredTimeDomain as u16, 321);
        assert_eq!(IeType::Metadata as u16, 322);
        assert_eq!(IeType::TrafficParameterMeasurementControlInformation as u16, 323);
        assert_eq!(IeType::TrafficParameterMeasurementReport as u16, 324);
        assert_eq!(IeType::TrafficParameterThreshold as u16, 325);
        assert_eq!(IeType::DlPeriodicity as u16, 326);
        assert_eq!(IeType::N6JitterMeasurement as u16, 327);
        assert_eq!(IeType::TrafficParameterMeasurementIndication as u16, 328);
        assert_eq!(IeType::UlPeriodicity as u16, 329);
        assert_eq!(IeType::MpquicControlInformation as u16, 330);
        assert_eq!(IeType::MpquicParameters as u16, 331);
        assert_eq!(IeType::MpquicAddressInformation as u16, 332);
        assert_eq!(IeType::TransportMode as u16, 333);
        assert_eq!(IeType::ProtocolDescription as u16, 334);
        assert_eq!(IeType::ReportingSuggestionInfo as u16, 335);
        assert_eq!(IeType::TlContainer as u16, 336);
        assert_eq!(IeType::MeasurementIndication as u16, 337);
        assert_eq!(IeType::HplmnSnssai as u16, 338);
        assert_eq!(IeType::MediaTransportProtocol as u16, 339);
        assert_eq!(IeType::RtpHeaderExtensionInformation as u16, 340);
        assert_eq!(IeType::RtpPayloadInformation as u16, 341);
        assert_eq!(IeType::RtpHeaderExtensionType as u16, 342);
        assert_eq!(IeType::RtpHeaderExtensionId as u16, 343);
        assert_eq!(IeType::RtpPayloadType as u16, 344);
        assert_eq!(IeType::RtpPayloadFormat as u16, 345);
        assert_eq!(IeType::ExtendedDlBufferingNotificationPolicy as u16, 346);
        assert_eq!(IeType::MtSdtControlInformation as u16, 347);
        assert_eq!(IeType::ReportingThresholds as u16, 348);
        assert_eq!(IeType::RtpHeaderExtensionAdditionalInformation as u16, 349);
        assert_eq!(IeType::MappedN6IpAddress as u16, 350);
        assert_eq!(IeType::N6RoutingInformation as u16, 351);
        assert_eq!(IeType::Uri as u16, 352);
        assert_eq!(IeType::UeLevelMeasurementsConfiguration as u16, 353);
        assert_eq!(IeType::N6DelayMeasurementProtocols as u16, 354);
        assert_eq!(IeType::N6DelayMeasurementControlInformation as u16, 355);
        assert_eq!(IeType::N6DelayMeasurementReport as u16, 356);
        assert_eq!(IeType::N6DelayMeasurementInformation as u16, 357);
        assert_eq!(IeType::MeasurementEndpointAddress as u16, 358);
        assert_eq!(IeType::OperatorConfigurableUpfCapability as u16, 359);
        assert_eq!(IeType::PacketInspectionFunctionality as u16, 360);
        assert_eq!(IeType::HeaderHandlingControlRule as u16, 361);
        assert_eq!(IeType::HeaderHandlingReportingControlInfo as u16, 362);
        assert_eq!(IeType::HeaderHandlingControlInformation as u16, 363);
        assert_eq!(IeType::HeaderDetectionReference as u16, 364);
        assert_eq!(IeType::HeaderDetectionSupportInformation as u16, 365);
        assert_eq!(IeType::ReportingEndpointId as u16, 366);
        assert_eq!(IeType::HeaderHandlingControlReference as u16, 367);
        assert_eq!(IeType::HeaderHandlingAction as u16, 368);
        assert_eq!(IeType::HeaderInformation as u16, 369);
        assert_eq!(IeType::HeaderValue as u16, 370);
        assert_eq!(IeType::HeaderHandlingCondition as u16, 371);
        assert_eq!(IeType::HeaderHandlingControlId as u16, 372);
        assert_eq!(IeType::HeaderHandlingControlRuleId as u16, 373);
        assert_eq!(IeType::OnPathN6ConnectionInformation as u16, 374);
        assert_eq!(IeType::MeasurementReportingType as u16, 375);
        assert_eq!(IeType::N6DelayMeasurementFailureInformation as u16, 376);
        assert_eq!(IeType::N6DelayMeasurementControlInformationId as u16, 377);
        assert_eq!(IeType::ProtocolSpecificConfigurationParameters as u16, 378);
        assert_eq!(IeType::MeasurementEndpointPortNumber as u16, 379);
        assert_eq!(IeType::HeaderHandlingReportingIndication as u16, 380);
        assert_eq!(IeType::SmfChangeReason as u16, 382);
        assert_eq!(IeType::ExtendedTransportLevelMarking as u16, 383);
        assert_eq!(IeType::PduSetImportance as u16, 384);
        assert_eq!(IeType::MoqControlInformation as u16, 385);
        assert_eq!(IeType::MoqInformation as u16, 386);
        assert_eq!(IeType::MoqRelayIpAddress as u16, 387);
        assert_eq!(IeType::MediaRelatedInformationTransferInfo as u16, 388);
        assert_eq!(IeType::ReportingControlInformation as u16, 389);
        assert_eq!(IeType::SecurityModeStamp as u16, 390);
        assert_eq!(IeType::HmacKeyStamp as u16, 391);
        assert_eq!(IeType::SecurityModeOwampTwamp as u16, 392);
        assert_eq!(IeType::KeyIdAndSharedSecretOwampTwamp as u16, 393);
        assert_eq!(IeType::RemainingDataReportingIndication as u16, 394);
        assert_eq!(IeType::ExpeditedTransferIndication as u16, 395);
        assert_eq!(IeType::SessionReflectorModeStamp as u16, 396);
        assert_eq!(IeType::PfdPartialFailureInformation as u16, 397);
        assert_eq!(IeType::TransportLevelMarkingIndications as u16, 398);
        assert_eq!(IeType::RedundantN3N9TransmissionInformation as u16, 399);
        assert_eq!(IeType::LocalN3N9TunnelInformation as u16, 400);
        assert_eq!(IeType::RemoteN3N9TunnelInformation as u16, 401);
        assert_eq!(IeType::BindingIndication as u16, 402);
        assert_eq!(IeType::PduSetImportanceForN6UnmarkedPdUs as u16, 403);
    }

}
