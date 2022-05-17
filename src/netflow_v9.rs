use std::collections::HashMap;

pub struct Parser {
    template_cache: HashMap<u16, Template>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    pub header: Header,
    pub flowsets: Vec<FlowSet>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Header {
    pub version: u16,
    pub count: u16,
    pub uptime: u32,
    pub unix_time: u32,
    pub packet_seq_num: u32,
    pub src_id: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FlowSet {
    pub header: FlowSetHeader,
    pub body: FlowSetBody,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FlowSetHeader {
    pub fid: u16,
    pub length: u16,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FlowSetBody {
    Template(Vec<Template>),
    Data(Vec<Record>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub id: u16,
    pub f_count: u16,
    pub fields: Vec<TemplateField>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TemplateField {
    pub ftype: u16,
    pub length: u16,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Record(pub Vec<Data>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Data {
    pub ftype: u16,
    pub data: Vec<u8>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            template_cache: HashMap::new(),
        }
    }

    pub fn parse(&mut self, buffer: &[u8]) -> Result<Packet, String> {
        // Parse packet header
        // Headers in netflow v9 are 160 bytes in total
        let header = self.read_header(&buffer[..20]);
        let buffer = &buffer[20..];

        let flowsets = self.parse_flowsets(buffer, header.count)?;

        Ok(Packet { header, flowsets })
    }

    fn parse_flowsets(
        &mut self,
        mut buffer: &[u8],
        mut count: u16,
    ) -> Result<Vec<FlowSet>, String> {
        // Keep parsing until we have read `header.count` flows
        let mut flowsets = Vec::with_capacity(count as usize);

        while count > 0 {
            let flowset = self.parse_flowset(buffer)?;

            let read_flows = {
                match &flowset.body {
                    FlowSetBody::Template(t) => t.len() as u16,
                    FlowSetBody::Data(d) => d.len() as u16,
                }
            };

            buffer = &buffer[flowset.header.length as usize..];

            count -= read_flows;
            flowsets.push(flowset);
        }

        Ok(flowsets)
    }

    fn parse_flowset(&mut self, buffer: &[u8]) -> Result<FlowSet, String> {
        let fid = read_u16(&buffer[0..2]);
        let length = read_u16(&buffer[2..4]);
        let header = FlowSetHeader { fid, length };

        let mut buffer = &buffer[4..length as usize];

        match fid {
            0 => {
                // Read all flowsets as templates
                let mut templates = vec![];

                while !buffer.is_empty() {
                    let template = self.parse_template(buffer);
                    // 2 bytes each for template id, field count and 4 for each
                    // field
                    buffer = &buffer[4 + template.f_count as usize * 4..];

                    // After reading a template, Also update template cache
                    let v = self
                        .template_cache
                        .entry(template.id)
                        .or_insert_with(|| template.clone());
                    *v = template.clone();

                    templates.push(template);
                }

                Ok(FlowSet {
                    header,
                    body: FlowSetBody::Template(templates),
                })
            }

            // Options flowset, not implemented
            1 => unimplemented!(),

            // Data Flowset
            v if v >= 256 => {
                let template = self
                    .template_cache
                    .get(&v)
                    .ok_or(format!("template with id {} not found in cache", v))?
                    .clone();

                let mut records = Vec::new();

                'outer: while !buffer.is_empty() {
                    let mut record = Vec::with_capacity(template.f_count as usize);

                    for field in template.fields.iter() {
                        if buffer.len() < field.length as usize {
                            println!("got padding of {} bytes", buffer.len());
                            break 'outer;
                        }

                        let data = self.parse_data(field, buffer);
                        buffer = &buffer[field.length as usize..];
                        record.push(data);
                    }

                    records.push(Record(record));
                }

                Ok(FlowSet {
                    header,
                    body: FlowSetBody::Data(records),
                })
            }

            _ => unreachable!(),
        }
    }

    fn parse_data(&self, field: &TemplateField, buffer: &[u8]) -> Data {
        let data = buffer[..field.length as usize].to_vec();

        Data {
            ftype: field.ftype,
            data,
        }
    }

    fn parse_template(&self, buffer: &[u8]) -> Template {
        let id = read_u16(&buffer[0..2]);
        let f_count = read_u16(&buffer[2..4]);
        let mut buffer = &buffer[4..];

        let mut fields = Vec::with_capacity(f_count as usize);

        for _ in 0..f_count {
            let ftype = read_u16(&buffer[..2]);
            let length = read_u16(&buffer[2..]);

            fields.push(TemplateField { ftype, length });

            buffer = &buffer[4..];
        }

        Template {
            id,
            fields,
            f_count,
        }
    }

    fn read_header(&self, buffer: &[u8]) -> Header {
        let version = read_u16(&buffer[0..2]);
        let count = read_u16(&buffer[2..4]);
        let uptime = read_u32(&buffer[4..8]);
        let unix_time = read_u32(&buffer[8..12]);
        let packet_seq_num = read_u32(&buffer[12..16]);
        let src_id = read_u32(&buffer[16..20]);

        Header {
            version,
            count,
            uptime,
            unix_time,
            packet_seq_num,
            src_id,
        }
    }
}

// read u32 from 4 u8 and advance buffer
const fn read_u32(buffer: &[u8]) -> u32 {
    u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]])
}

// read u16 from 2 u8 and advance buffer
const fn read_u16(buffer: &[u8]) -> u16 {
    u16::from_be_bytes([buffer[0], buffer[1]])
}
