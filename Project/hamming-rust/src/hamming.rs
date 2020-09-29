use num::pow::pow;

pub mod encode{
    pub fn to_binary_representation(text:&String)->String{
        let mut text_in_binary = "".to_string();

        for c in text.clone().into_bytes(){
           text_in_binary += &format!("0{:b} ", c); 
        }
        text_in_binary
    }
    pub fn encode_data(text:&String)-> Vec<u64>{
        let mut byte_stream: Vec<u8> = Vec::new();
        let mut out_stream: Vec<u64> = Vec::new();
        let mut block:u64 = 0;
        let mut bit:u8 = 0;
        let mut cnt:u16 = 0;
        let mut cnt_1bit:u8 = 0;
        let mut parity:u8 = 0;

        byte_stream = text.clone().into_bytes();

        for mut b in byte_stream.into_iter() {
            // extract every single bit in byte
            let mut a = 0; 
            while a < 8 as u8 {
                bit = (b & 128) >> 7;
                b <<= 1;
                // println!("{}, {}",bit,b);
                match bit {
                    0 => {} // do nothing
                    1 => {
                        block |= num::pow::pow(cnt,2usize)as u64;
                        cnt_1bit ^= 1;
                        parity ^= num::pow::pow(cnt,2usize)as u8;
                    }
                    _ => {panic!("Unexpected Error");}
                }
                a += 1;
                cnt = (cnt+1)%64;
                match cnt {
                    0 => {
                        // parity bit 
                        let mut a = 6;
                        while a > 0 {
                            bit = parity & 1;
                            parity >>= 1;
                            match bit {
                                0 => {}
                                1 => {
                                    block |= num::pow::pow(64-a-1,2usize)as u64;
                                    cnt_1bit ^= 1;
                                    block = 0;
                                }
                                _ => {panic!("Error in: parity bit");}
                            }
                            a -= 1;
                        }
                        if cnt_1bit == 1 {
                            block |= num::pow::pow(63,2usize)as u64;
                        }
                        out_stream.push(block);
                        block = 0;
                        cnt = 3;
                    }
                    1|2 => {cnt=3;}
                    4|8|16|32 => {cnt+=1;}
                    _ => {} // do nothing
                }
            }
            // println!("###NewByte###");
        }
        // need padding 
        println!("------------");
        //println!("{:?}",out_stream);
        out_stream
    }
}
