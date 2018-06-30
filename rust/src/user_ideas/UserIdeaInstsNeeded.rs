
use banda::src::storage::field_key;
struct UserIdeaInstsNeeded{
    key: field_key;
    completes: HashMap<field_key, UserIdeaComplete>;
}

impl UserIdeaInstsNeeded {
    fn new(complete: &UserIdeaComplete, needed: data_field){
        &self.completes.put(complete.total_key, compelete);
        &self.key=1;
    }

    fn computeKeys{
        for x:String in needed{
            key*=x;
        }
    }

}
