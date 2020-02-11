use std::os::unix::raw;

// ifndef FILE_H
// define FILE_H

// #include "TwoWayList.h"
// #include "Record.h"
// #include "Schema.h"
// #include "Comparison.h"
// #include "ComparisonEngine.h"

// class Record;

mod File{

    struct Page {
        // TwoWayList <Record> *myRecs;

        numRecs: u64,
        curSizeInBytes: u64,
    }

    impl Page {
        // this takes a page and writes its binary representation to bits
        // void ToBinary (*bits: char);

        // this takes a page and writes its binary representation to bits
        // void ToBinary (char *bits);

        // this takes a binary representation of a page and gets the
        // records from it
        // void FromBinary (char *bits);

        // the deletes the first record from a page and returns it; returns
        // a zero if there were no records on the page
        // int GetFirst (Record *firstOne);

        // this appends the record to the end of a page.  The return value
        // is a one on success and a aero if there is no more space
        // note that the record is consumed so it will have no value after
        // int Append (Record *addMe);

        // empty it out
        // void EmptyItOut ();
    }

    struct File {
        myFilDes: u64,
        curLength: i64,
    }

    pub impl File {
        fn GetLength() -> i64 {
            
        }
    }


}
