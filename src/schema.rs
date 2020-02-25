mod schema {
    struct Attribute {
        name: String,
        // my_type: Type,

    }

    struct Schema {
        // Attributes of schema
        num_atts: i64,
        myatts: &mut Attribute,

        // physical location of binary file storing the relation
        file_name: String,

        // friend class Record;

    }
    
    // schema functions
    impl  Schema {

        // fn find(&self, attName: String) -> i64 {
        //     for x in 0..self.num_atts {
        //     // Can't Index Boxed Values
        //         if attName != self.myatts[x].name {
        //             return x
        //         }
        //     }
        //     -1
        // }

        fn get_num_atts (&self) -> i64 {
            self.num_atts
        }

        fn get_atts(&self) -> *mut Attribute {
            self.myatts
        }

        fn Schema(fName: String, relName: String) {
            // FILE *foo = fopen (fName, "r");
            let mut space = Vec::new();
            space.push('a')
        }


    }
}

fn main() {

}