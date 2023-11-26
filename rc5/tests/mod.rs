/// generated using the code in: https://www.ietf.org/archive/id/draft-krovetz-rc6-rc5-vectors-00.txt
#[cfg(test)]
mod tests {
    use cipher::consts::*;
    use cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
    use rc5::RC5;

    #[test]
    fn enc_dec_8_12_4() {
        let key = [0x00, 0x01, 0x02, 0x03];

        let pt = [0x00, 0x01];
        let ct = [0x21, 0x2A];

        let rc5 = <RC5<u8, U12, U4> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }

    #[test]
    fn enc_dec_16_16_8() {
        let key = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

        let pt = [0x00, 0x01, 0x02, 0x03];
        let ct = [0x23, 0xA8, 0xD7, 0x2E];

        let rc5 = <RC5<u16, U16, U8> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }

    #[test]
    fn enc_dec_32_12_16() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let pt = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let ct = [0xC8, 0xD3, 0xB3, 0xC4, 0x86, 0x70, 0x0C, 0xFA];

        let rc5 = <RC5<u32, U12, U16> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }

    #[test]
    fn enc_dec_32_16_16() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];

        let pt = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let ct = [0x3E, 0x2E, 0x95, 0x35, 0x70, 0x27, 0xD8, 0x96];

        let rc5 = <RC5<u32, U16, U16> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }

    #[test]
    fn enc_dec_64_24_24() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ];

        let pt = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let ct = [
            0xA4, 0x67, 0x72, 0x82, 0x0E, 0xDB, 0xCE, 0x02, 0x35, 0xAB, 0xEA, 0x32, 0xAE, 0x71,
            0x78, 0xDA,
        ];

        let rc5 = <RC5<u64, U24, U24> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }

    #[test]
    fn enc_dec_128_28_32() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];

        let pt = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let ct = [
            0xEC, 0xA5, 0x91, 0x09, 0x21, 0xA4, 0xF4, 0xCF, 0xDD, 0x7A, 0xD7, 0xAD, 0x20, 0xA1,
            0xFC, 0xBA, 0x06, 0x8E, 0xC7, 0xA7, 0xCD, 0x75, 0x2D, 0x68, 0xFE, 0x91, 0x4B, 0x7F,
            0xE1, 0x80, 0xB4, 0x40,
        ];

        let rc5 = <RC5<u128, U28, U32> as KeyInit>::new_from_slice(&key).unwrap();

        let mut block = GenericArray::clone_from_slice(&pt);
        rc5.encrypt_block(&mut block);

        assert_eq!(ct, block[..]);

        rc5.decrypt_block(&mut block);
        assert_eq!(pt, block[..]);
    }
}
