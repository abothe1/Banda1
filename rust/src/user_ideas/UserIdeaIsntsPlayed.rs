struct UserIdeaPlayed{
  key: field_key;
  completes: HashMap<field_key, UserIdeaComplete>;

}

impl UserIdeaPlayed {
    fn new(complete: &UserIdeaComplete, insts_played: data_field){
        &self.completes.put(complete.total_key, coemplete);

    }
    fn computeKeys(){
        for x:String in insts_played{
            &self.key*=x;
        }
    }
}
