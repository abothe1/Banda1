
use rust::storage::data_field
struct User{
    total_key: u64,
    g_key: u64,
    g_s_key: u64,
    g_vals: [u8],
    insts_needed_key:u64,
    insts_played_key:u64,
    isnts_played: [string],
    isnts_needed: [string],
    latt: u64,
    longit: u64,
    maxDist: u32,
    user_name:&str,
    password: &str,
    email: String,
    phone: String

}

impl User(user_name: String, password: String, genreBank: [String]){
    &self.user_name=user_name;
    &self.password = password;
    let mut temp_g_key = 1;
    fn computeKeys(){
        for x in &self.g_vals {
            let mut df1= data_field::new(field_id:genreBank[x],Preference:x);

        }
    }
}
