use netflow_zmq::netflow_v9::*;

#[test]
fn multiple_flowsets() {
    let buffer = [
        0x00, 0x09, 0x00, 0x12, 0xc3, 0x0b, 0xfb, 0x2a, 0x53, 0xe0, 0xbd, 0x96, 0xa5, 0xba, 0xb7,
        0xef, 0x00, 0x00, 0x00, 0xc8, 0x00, 0x00, 0x00, 0x90, 0x01, 0x03, 0x00, 0x09, 0x00, 0x08,
        0x00, 0x04, 0x00, 0xe1, 0x00, 0x04, 0x00, 0xea, 0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x00,
        0xe6, 0x00, 0x01, 0x01, 0x43, 0x00, 0x08, 0x01, 0x69, 0x00, 0x02, 0x01, 0x6b, 0x00, 0x02,
        0x01, 0x6c, 0x00, 0x02, 0x01, 0x02, 0x00, 0x02, 0x01, 0x1b, 0x00, 0x04, 0x00, 0xe6, 0x00,
        0x01, 0x01, 0x01, 0x00, 0x08, 0x00, 0x08, 0x00, 0x04, 0x00, 0xe1, 0x00, 0x04, 0x00, 0x07,
        0x00, 0x02, 0x00, 0xe3, 0x00, 0x02, 0x00, 0xea, 0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x00,
        0xe6, 0x00, 0x01, 0x01, 0x43, 0x00, 0x08, 0x01, 0x00, 0x00, 0x0c, 0x00, 0x08, 0x00, 0x04,
        0x00, 0xe1, 0x00, 0x04, 0x00, 0x0c, 0x00, 0x04, 0x00, 0xe2, 0x00, 0x04, 0x00, 0x07, 0x00,
        0x02, 0x00, 0xe3, 0x00, 0x02, 0x00, 0x0b, 0x00, 0x02, 0x00, 0xe4, 0x00, 0x02, 0x00, 0xea,
        0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x00, 0xe6, 0x00, 0x01, 0x01, 0x43, 0x00, 0x08, 0x01,
        0x00, 0x02, 0x18, 0x64, 0x40, 0x9c, 0x93, 0xb9, 0x06, 0x19, 0x9c, 0x42, 0xdc, 0x98, 0x13,
        0x42, 0xdc, 0x98, 0x13, 0xd5, 0x40, 0x7b, 0xc8, 0x01, 0xbb, 0x01, 0xbb, 0x00, 0x00, 0x00,
        0x00, 0x06, 0x01, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe5, 0x64, 0x40, 0xa1, 0x19,
        0xb9, 0x06, 0x19, 0xa1, 0x5b, 0xda, 0x7a, 0x94, 0x5b, 0xda, 0x7a, 0x94, 0xcf, 0x6b, 0xdf,
        0x60, 0x00, 0x50, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x01, 0x47,
        0xa5, 0xe4, 0x92, 0xe5, 0x64, 0x40, 0x31, 0x94, 0xb9, 0x06, 0x19, 0x31, 0x02, 0x85, 0x7a,
        0x86, 0x02, 0x85, 0x7a, 0x86, 0xce, 0xe4, 0xb6, 0x74, 0x1a, 0xe1, 0x1a, 0xe1, 0x00, 0x00,
        0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe5, 0x64, 0x40, 0x31,
        0x94, 0xb9, 0x06, 0x19, 0x31, 0x5f, 0x98, 0x06, 0x43, 0x5f, 0x98, 0x06, 0x43, 0xce, 0xe2,
        0xb6, 0x75, 0x1a, 0xe2, 0x1a, 0xe2, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x01,
        0x47, 0xa5, 0xe4, 0x92, 0xe5, 0x64, 0x40, 0xd9, 0x68, 0xb9, 0x06, 0x19, 0xd9, 0xb9, 0x0c,
        0xf1, 0x9b, 0xb9, 0x0c, 0xf1, 0x9b, 0x04, 0x19, 0x78, 0xb0, 0x00, 0x50, 0x00, 0x50, 0x00,
        0x00, 0x00, 0x00, 0x06, 0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe5, 0x64, 0x40,
        0x31, 0x94, 0xb9, 0x06, 0x19, 0x31, 0xb2, 0x2f, 0x7a, 0xa0, 0xb2, 0x2f, 0x7a, 0xa0, 0xce,
        0xe3, 0xb6, 0x76, 0x1a, 0xe1, 0x1a, 0xe1, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00,
        0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6, 0x64, 0x40, 0xc5, 0xc9, 0xb9, 0x06, 0x19, 0xc5, 0x57,
        0xf0, 0xa2, 0x4f, 0x57, 0xf0, 0xa2, 0x4f, 0xa0, 0x5d, 0x1a, 0xe0, 0x00, 0x50, 0x00, 0x50,
        0x00, 0x00, 0x00, 0x00, 0x06, 0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6, 0x64,
        0x40, 0x95, 0x02, 0xb9, 0x06, 0x19, 0x95, 0xb4, 0x04, 0x10, 0x79, 0xb4, 0x04, 0x10, 0x79,
        0x04, 0x43, 0x5a, 0x63, 0x2c, 0x79, 0x2c, 0x79, 0x00, 0x00, 0x00, 0x00, 0x11, 0x01, 0x00,
        0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6, 0x64, 0x40, 0x9a, 0x67, 0xb9, 0x06, 0x19, 0x9a,
        0x57, 0xf0, 0x87, 0xd2, 0x57, 0xf0, 0x87, 0xd2, 0xfc, 0x39, 0x3a, 0xc2, 0x00, 0x50, 0x00,
        0x50, 0x00, 0x00, 0x00, 0x00, 0x06, 0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6,
        0x64, 0x40, 0xf4, 0x3b, 0xb9, 0x06, 0x19, 0xf4, 0xad, 0xc2, 0x20, 0x81, 0xad, 0xc2, 0x20,
        0x81, 0xb2, 0x96, 0x5a, 0xc9, 0x01, 0xbb, 0x01, 0xbb, 0x00, 0x00, 0x00, 0x00, 0x06, 0x02,
        0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6, 0x64, 0x40, 0xb9, 0x26, 0xb9, 0x06, 0x19,
        0xb9, 0x56, 0x3b, 0xed, 0x27, 0x56, 0x3b, 0xed, 0x27, 0x51, 0xa8, 0x69, 0x91, 0xc8, 0xd5,
        0xc8, 0xd5, 0x00, 0x00, 0x00, 0x00, 0x11, 0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92,
        0xe6, 0x64, 0x40, 0xc6, 0x2c, 0xb9, 0x06, 0x19, 0xc6, 0xd9, 0x76, 0x5f, 0x41, 0xd9, 0x76,
        0x5f, 0x41, 0xd3, 0xf3, 0xd2, 0xce, 0x53, 0xb1, 0x53, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x06,
        0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6, 0x64, 0x40, 0x52, 0xb2, 0xb9, 0x06,
        0x19, 0x52, 0x57, 0xf5, 0xc4, 0x6a, 0x57, 0xf5, 0xc4, 0x6a, 0x97, 0x94, 0x99, 0xa0, 0x00,
        0x50, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4,
        0x92, 0xe6, 0x64, 0x40, 0x50, 0x95, 0xb9, 0x06, 0x19, 0x50, 0xbc, 0x2b, 0x6f, 0xa0, 0xbc,
        0x2b, 0x6f, 0xa0, 0xfb, 0xfb, 0x5c, 0x5c, 0x58, 0x7c, 0x58, 0x7c, 0x00, 0x00, 0x00, 0x00,
        0x06, 0x02, 0x00, 0x00, 0x01, 0x47, 0xa5, 0xe4, 0x92, 0xe6,
    ];

    let mut parser = Parser::new();

    let packet = parser.parse(&buffer);

    assert_eq!(
        packet,
        Ok(Packet {
            header: Header {
                version: 9,
                count: 18,
                uptime: 3272342314,
                unix_time: 1407237526,
                packet_seq_num: 2780477423,
                src_id: 200
            },
            flowsets: vec![
                FlowSet {
                    header: FlowSetHeader {
                        fid: 0,
                        length: 144
                    },
                    body: FlowSetBody::Template(vec![
                        Template {
                            id: 259,
                            f_count: 9,
                            fields: vec![
                                TemplateField {
                                    ftype: 8,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 225,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 234,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 4,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 230,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 323,
                                    length: 8
                                },
                                TemplateField {
                                    ftype: 361,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 363,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 364,
                                    length: 2
                                }
                            ]
                        },
                        Template {
                            id: 258,
                            f_count: 2,
                            fields: vec![
                                TemplateField {
                                    ftype: 283,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 230,
                                    length: 1
                                }
                            ]
                        },
                        Template {
                            id: 257,
                            f_count: 8,
                            fields: vec![
                                TemplateField {
                                    ftype: 8,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 225,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 7,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 227,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 234,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 4,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 230,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 323,
                                    length: 8
                                }
                            ]
                        },
                        Template {
                            id: 256,
                            f_count: 12,
                            fields: vec![
                                TemplateField {
                                    ftype: 8,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 225,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 12,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 226,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 7,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 227,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 11,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 228,
                                    length: 2
                                },
                                TemplateField {
                                    ftype: 234,
                                    length: 4
                                },
                                TemplateField {
                                    ftype: 4,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 230,
                                    length: 1
                                },
                                TemplateField {
                                    ftype: 323,
                                    length: 8
                                }
                            ]
                        }
                    ])
                },
                FlowSet {
                    header: FlowSetHeader {
                        fid: 256,
                        length: 536
                    },
                    body: FlowSetBody::Data(vec![
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 156, 147]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 156]
                            },
                            Data {
                                ftype: 12,
                                data: vec![66, 220, 152, 19]
                            },
                            Data {
                                ftype: 226,
                                data: vec![66, 220, 152, 19]
                            },
                            Data {
                                ftype: 7,
                                data: vec![213, 64]
                            },
                            Data {
                                ftype: 227,
                                data: vec![123, 200]
                            },
                            Data {
                                ftype: 11,
                                data: vec![1, 187]
                            },
                            Data {
                                ftype: 228,
                                data: vec![1, 187]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 229]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 161, 25]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 161]
                            },
                            Data {
                                ftype: 12,
                                data: vec![91, 218, 122, 148]
                            },
                            Data {
                                ftype: 226,
                                data: vec![91, 218, 122, 148]
                            },
                            Data {
                                ftype: 7,
                                data: vec![207, 107]
                            },
                            Data {
                                ftype: 227,
                                data: vec![223, 96]
                            },
                            Data {
                                ftype: 11,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 228,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 229]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 49, 148]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 49]
                            },
                            Data {
                                ftype: 12,
                                data: vec![2, 133, 122, 134]
                            },
                            Data {
                                ftype: 226,
                                data: vec![2, 133, 122, 134]
                            },
                            Data {
                                ftype: 7,
                                data: vec![206, 228]
                            },
                            Data {
                                ftype: 227,
                                data: vec![182, 116]
                            },
                            Data {
                                ftype: 11,
                                data: vec![26, 225]
                            },
                            Data {
                                ftype: 228,
                                data: vec![26, 225]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 229]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 49, 148]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 49]
                            },
                            Data {
                                ftype: 12,
                                data: vec![95, 152, 6, 67]
                            },
                            Data {
                                ftype: 226,
                                data: vec![95, 152, 6, 67]
                            },
                            Data {
                                ftype: 7,
                                data: vec![206, 226]
                            },
                            Data {
                                ftype: 227,
                                data: vec![182, 117]
                            },
                            Data {
                                ftype: 11,
                                data: vec![26, 226]
                            },
                            Data {
                                ftype: 228,
                                data: vec![26, 226]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 229]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 217, 104]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 217]
                            },
                            Data {
                                ftype: 12,
                                data: vec![185, 12, 241, 155]
                            },
                            Data {
                                ftype: 226,
                                data: vec![185, 12, 241, 155]
                            },
                            Data {
                                ftype: 7,
                                data: vec![4, 25]
                            },
                            Data {
                                ftype: 227,
                                data: vec![120, 176]
                            },
                            Data {
                                ftype: 11,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 228,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 229]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 49, 148]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 49]
                            },
                            Data {
                                ftype: 12,
                                data: vec![178, 47, 122, 160]
                            },
                            Data {
                                ftype: 226,
                                data: vec![178, 47, 122, 160]
                            },
                            Data {
                                ftype: 7,
                                data: vec![206, 227]
                            },
                            Data {
                                ftype: 227,
                                data: vec![182, 118]
                            },
                            Data {
                                ftype: 11,
                                data: vec![26, 225]
                            },
                            Data {
                                ftype: 228,
                                data: vec![26, 225]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 197, 201]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 197]
                            },
                            Data {
                                ftype: 12,
                                data: vec![87, 240, 162, 79]
                            },
                            Data {
                                ftype: 226,
                                data: vec![87, 240, 162, 79]
                            },
                            Data {
                                ftype: 7,
                                data: vec![160, 93]
                            },
                            Data {
                                ftype: 227,
                                data: vec![26, 224]
                            },
                            Data {
                                ftype: 11,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 228,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 149, 2]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 149]
                            },
                            Data {
                                ftype: 12,
                                data: vec![180, 4, 16, 121]
                            },
                            Data {
                                ftype: 226,
                                data: vec![180, 4, 16, 121]
                            },
                            Data {
                                ftype: 7,
                                data: vec![4, 67]
                            },
                            Data {
                                ftype: 227,
                                data: vec![90, 99]
                            },
                            Data {
                                ftype: 11,
                                data: vec![44, 121]
                            },
                            Data {
                                ftype: 228,
                                data: vec![44, 121]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![17]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 154, 103]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 154]
                            },
                            Data {
                                ftype: 12,
                                data: vec![87, 240, 135, 210]
                            },
                            Data {
                                ftype: 226,
                                data: vec![87, 240, 135, 210]
                            },
                            Data {
                                ftype: 7,
                                data: vec![252, 57]
                            },
                            Data {
                                ftype: 227,
                                data: vec![58, 194]
                            },
                            Data {
                                ftype: 11,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 228,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 244, 59]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 244]
                            },
                            Data {
                                ftype: 12,
                                data: vec![173, 194, 32, 129]
                            },
                            Data {
                                ftype: 226,
                                data: vec![173, 194, 32, 129]
                            },
                            Data {
                                ftype: 7,
                                data: vec![178, 150]
                            },
                            Data {
                                ftype: 227,
                                data: vec![90, 201]
                            },
                            Data {
                                ftype: 11,
                                data: vec![1, 187]
                            },
                            Data {
                                ftype: 228,
                                data: vec![1, 187]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 185, 38]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 185]
                            },
                            Data {
                                ftype: 12,
                                data: vec![86, 59, 237, 39]
                            },
                            Data {
                                ftype: 226,
                                data: vec![86, 59, 237, 39]
                            },
                            Data {
                                ftype: 7,
                                data: vec![81, 168]
                            },
                            Data {
                                ftype: 227,
                                data: vec![105, 145]
                            },
                            Data {
                                ftype: 11,
                                data: vec![200, 213]
                            },
                            Data {
                                ftype: 228,
                                data: vec![200, 213]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![17]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 198, 44]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 198]
                            },
                            Data {
                                ftype: 12,
                                data: vec![217, 118, 95, 65]
                            },
                            Data {
                                ftype: 226,
                                data: vec![217, 118, 95, 65]
                            },
                            Data {
                                ftype: 7,
                                data: vec![211, 243]
                            },
                            Data {
                                ftype: 227,
                                data: vec![210, 206]
                            },
                            Data {
                                ftype: 11,
                                data: vec![83, 177]
                            },
                            Data {
                                ftype: 228,
                                data: vec![83, 177]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 82, 178]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 82]
                            },
                            Data {
                                ftype: 12,
                                data: vec![87, 245, 196, 106]
                            },
                            Data {
                                ftype: 226,
                                data: vec![87, 245, 196, 106]
                            },
                            Data {
                                ftype: 7,
                                data: vec![151, 148]
                            },
                            Data {
                                ftype: 227,
                                data: vec![153, 160]
                            },
                            Data {
                                ftype: 11,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 228,
                                data: vec![0, 80]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![1]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ]),
                        Record(vec![
                            Data {
                                ftype: 8,
                                data: vec![100, 64, 80, 149]
                            },
                            Data {
                                ftype: 225,
                                data: vec![185, 6, 25, 80]
                            },
                            Data {
                                ftype: 12,
                                data: vec![188, 43, 111, 160]
                            },
                            Data {
                                ftype: 226,
                                data: vec![188, 43, 111, 160]
                            },
                            Data {
                                ftype: 7,
                                data: vec![251, 251]
                            },
                            Data {
                                ftype: 227,
                                data: vec![92, 92]
                            },
                            Data {
                                ftype: 11,
                                data: vec![88, 124]
                            },
                            Data {
                                ftype: 228,
                                data: vec![88, 124]
                            },
                            Data {
                                ftype: 234,
                                data: vec![0, 0, 0, 0]
                            },
                            Data {
                                ftype: 4,
                                data: vec![6]
                            },
                            Data {
                                ftype: 230,
                                data: vec![2]
                            },
                            Data {
                                ftype: 323,
                                data: vec![0, 0, 1, 71, 165, 228, 146, 230]
                            }
                        ])
                    ])
                }
            ]
        })
    );
}

#[test]
fn multiple_templates() {
    let buffer = [
        0x00, 0x09, 0x00, 0x02, 0x05, 0x08, 0x50, 0x70, 0x62, 0x81, 0x4e, 0xd8, 0x00, 0x00, 0xfe,
        0xca, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0x01, 0x00, 0x00, 0x1a, 0x00, 0x15,
        0x00, 0x04, 0x00, 0x16, 0x00, 0x04, 0x00, 0x02, 0x00, 0x04, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x0a, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x04, 0x00, 0x08, 0x00, 0x04, 0x00, 0x0c, 0x00, 0x04,
        0x00, 0x04, 0x00, 0x01, 0x00, 0x05, 0x00, 0x01, 0x00, 0x07, 0x00, 0x02, 0x00, 0x0b, 0x00,
        0x02, 0x00, 0x0f, 0x00, 0x04, 0x00, 0x0d, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x06,
        0x00, 0x01, 0x00, 0x22, 0x00, 0x04, 0x00, 0x23, 0x00, 0x01, 0x00, 0x50, 0x00, 0x06, 0x00,
        0x38, 0x00, 0x06, 0x00, 0x39, 0x00, 0x06, 0x00, 0x51, 0x00, 0x06, 0x00, 0xe1, 0x00, 0x04,
        0x00, 0xe2, 0x00, 0x04, 0x00, 0xe3, 0x00, 0x02, 0x00, 0xe4, 0x00, 0x02, 0x01, 0x01, 0x00,
        0x19, 0x00, 0x3c, 0x00, 0x01, 0x00, 0x1b, 0x00, 0x10, 0x00, 0x1d, 0x00, 0x01, 0x00, 0x0a,
        0x00, 0x04, 0x00, 0x1c, 0x00, 0x10, 0x00, 0x1e, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x04, 0x00,
        0x3e, 0x00, 0x10, 0x00, 0x04, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x22, 0x00, 0x04,
        0x00, 0x23, 0x00, 0x01, 0x00, 0x05, 0x00, 0x01, 0x00, 0x07, 0x00, 0x02, 0x00, 0x0b, 0x00,
        0x02, 0x00, 0x1f, 0x00, 0x04, 0x00, 0x40, 0x00, 0x04, 0x00, 0x15, 0x00, 0x04, 0x00, 0x16,
        0x00, 0x04, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x04, 0x00, 0x50, 0x00, 0x06, 0x00,
        0x38, 0x00, 0x06, 0x00, 0x39, 0x00, 0x06, 0x00, 0x51, 0x00, 0x06,
    ];

    let mut parser = Parser::new();
    let packet = parser.parse(&buffer);

    assert_eq!(
        packet,
        Ok(Packet {
            header: Header {
                version: 9,
                count: 2,
                uptime: 84430960,
                unix_time: 1652641496,
                packet_seq_num: 65226,
                src_id: 0,
            },
            flowsets: vec![FlowSet {
                header: FlowSetHeader {
                    fid: 0,
                    length: 216,
                },
                body: FlowSetBody::Template(vec![
                    Template {
                        id: 256,
                        f_count: 26,
                        fields: vec![
                            TemplateField {
                                ftype: 21,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 22,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 2,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 1,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 10,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 14,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 8,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 12,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 4,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 5,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 7,
                                length: 2,
                            },
                            TemplateField {
                                ftype: 11,
                                length: 2,
                            },
                            TemplateField {
                                ftype: 15,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 13,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 9,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 6,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 34,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 35,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 80,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 56,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 57,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 81,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 225,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 226,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 227,
                                length: 2,
                            },
                            TemplateField {
                                ftype: 228,
                                length: 2,
                            },
                        ],
                    },
                    Template {
                        id: 257,
                        f_count: 25,
                        fields: vec![
                            TemplateField {
                                ftype: 60,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 27,
                                length: 16,
                            },
                            TemplateField {
                                ftype: 29,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 10,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 28,
                                length: 16,
                            },
                            TemplateField {
                                ftype: 30,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 14,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 62,
                                length: 16,
                            },
                            TemplateField {
                                ftype: 4,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 6,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 34,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 35,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 5,
                                length: 1,
                            },
                            TemplateField {
                                ftype: 7,
                                length: 2,
                            },
                            TemplateField {
                                ftype: 11,
                                length: 2,
                            },
                            TemplateField {
                                ftype: 31,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 64,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 21,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 22,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 1,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 2,
                                length: 4,
                            },
                            TemplateField {
                                ftype: 80,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 56,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 57,
                                length: 6,
                            },
                            TemplateField {
                                ftype: 81,
                                length: 6,
                            },
                        ],
                    },
                ],),
            },],
        },)
    );
}
