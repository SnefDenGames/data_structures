/// anything for the dictionary data structure
pub mod dictionary {
    /// a list of `Datapair`s
    pub struct Dictionary<D> {
        /// the `Datapair`s
        pairs:Vec<Datapair<D>>
    }
    impl<D> Dictionary<D> {
        /// create a new `Dictionary` for D
        /// 
        /// # Return
        /// ```
        /// Dictionary<D>
        /// ```
        /// 
        /// # Examples
        /// ```
        /// let dictonary:Dictionary<String> = Dictionary::new();
        /// ```
        /// 
        /// # Coution
        /// The type for the data has to be set.
        /// Wrong example:
        /// ```
        /// let dictionary = Dictionary::new();
        /// ```
        /// This gives an error, because the type for the data is not clear.
        pub fn new() -> Self {
            Dictionary {
                pairs:Vec::new()
            }
        }
        /// add data to the `Dictionary`
        /// 
        /// # Params
        /// ```
        /// p: Datapair<D>
        /// ```
        /// 
        /// # Example
        /// ```
        /// dictionary.add(String::from("Lola"),String::from("Sky"));
        /// ```
        pub fn add(&mut self,k:String,d:D) {
            self.pairs.push(Datapair::new(k,d));
        }
        /// add a pair to the `Dictionary`
        /// 
        /// # Params
        /// ```
        /// k: String // the key
        /// d: D // the data
        /// ```
        /// 
        /// # Example
        /// ```
        /// let pair: Datapair<String> = Datapair::new(String::from("Marie"),String::from("Karate"))
        /// dictionary.add_pair(pair);   
        /// ```
        pub fn add_pair(&mut self,p: Datapair<D>) {
            self.pairs.push(p);
        }
        /// removes a pair from the `Dictionary`
        /// 
        /// # Params
        /// ```
        /// k: String // the key
        /// ```
        /// 
        /// # Example
        /// ```
        /// dictionary.remove(String::from("Lola"));
        /// ```
        pub fn remove(&mut self,k:String) {
            self.pairs.retain(|x| x.get_key() != &k);
        }
        /// changes the data for the given key
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// d:D // the new data
        /// ```
        /// 
        /// # Example
        /// ```
        /// dictionary.change_data(String::from("Maxime"),String::from("Swimming"));
        /// ```
        pub fn change_data(&mut self,k:String,d:D) {
            let l = *&self.pairs.len();
            let mut i = 0;
            while i<l {
                if &self.pairs[i].get_key() == &&k {
                    &self.pairs[i].set_data(d);
                    break;
                }
                i = i+1;
            }
        }
        /// changes the key for the given old key
        /// 
        /// # Params
        /// ```
        /// ok:String // the old key
        /// nk:String // the new key
        /// ```
        /// 
        /// # Example
        /// ```
        /// dictionary.change_key(String::from("Anna"),String::from("Anna-Lena"));
        /// ```
        pub fn change_key(&mut self,ok:String,nk:String) {
            let l = *&self.pairs.len();
            let mut i = 0;
            while i<l {
                if &self.pairs[i].get_key() == &&ok {
                    &self.pairs[i].set_key(nk);
                    break;
                }
                i = i+1;
            }
        }
        /// get a specific `Datapair` for the given key
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// ```
        /// 
        /// # Return
        /// ```
        /// &Dictonary<D>
        /// ```
        /// 
        /// # Example
        /// ```
        /// let pair:Datapair<String> = dictionary.get_pair(String::from("Anna"));
        /// ```
        pub fn get_pair(&self, k:String) -> &Datapair<D> {
            let l = *&self.pairs.len();
            let mut i = 0;
            while i<l {
                if &self.pairs[i].get_key() == &&k {
                    break;
                }
                i = i+1;
            }
            &self.pairs[i]
        }
        /// get the data for the given key
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// ```
        /// 
        /// # Return
        /// ```
        /// &D
        /// ```
        /// 
        /// # Example
        /// ```
        /// let hobby = dictionary.get_data(String::from("Anna"));
        /// ```
        pub fn get_data(&self, k:String) -> &D {
            let l = *&self.pairs.len();
            let mut i = 0;
            while i<l {
                if &self.pairs[i].get_key() == &&k {
                    break;
                }
                i = i+1;
            }
            &self.pairs[i].get_data()
        }
        /// clears the `Dictionary`
        /// 
        /// # Example
        /// ```
        /// dictionary.clear();
        /// ```
        pub fn clear(&mut self) {
            self.pairs  =   Vec::new();
        }
        /// gives you how many pairs are in the `Dictionary`
        /// 
        /// # Returns
        /// ```
        /// usize
        /// ```
        /// 
        /// # Example
        /// ```
        /// let size:usize = dictionary.size();
        /// ```
        pub fn size(&self) -> usize {
            *&self.pairs.len()
        }
    }
    impl<D: Clone> Clone for Dictionary<D> {
        fn clone(&self) -> Self {
            Dictionary {
                pairs: self.pairs.clone()
            }
        }
    }
    impl<D: std::fmt::Display + Clone> std::fmt::Display for Dictionary<D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let open = String::from("\u{007B}");
            let close = String::from("\u{007D}");
            let mut text = String::new();
            for element in &self.pairs {
                if element.get_key() == self.pairs[0].get_key() {
                    text = format!("{}\n{}",text,element);
                } else if element.get_key() == self.pairs[self.size()-1].get_key() {
                    text = format!("{}\n{}\n",text,element);
                } else {
                    text = format!("{},\n\t{}",text,element);
                }
            }
            write!(f, "{}{}{}", open, text, close)
        }
    }
    impl<D: PartialEq> PartialEq for Dictionary<D> {
        fn eq(&self, other: &Self) -> bool {
            if self.pairs.len() == other.pairs.len() {
                for i in 0..self.pairs.len() {
                    if self.pairs[i] != other.pairs[i] {
                        return false;
                    }
                }
                return true;
            }
            return false;
        }
        fn ne(&self, other: &Self) -> bool {
            if self.pairs.len() == other.pairs.len() {
                for i in 0..self.pairs.len() {
                    if self.pairs[i] != other.pairs[i] {
                        return true;
                    }
                }
                return false;
            }
            return true;
        }
    }



    /// a `Datapair`
    pub struct Datapair<D> {
        /// the key of the pair
        key:String,
        /// the data of the pair
        data:D
    }
    impl<D> Datapair<D> {
        /// create a new `Datapair` with the given key and data
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// d:D // the data
        /// ```
        /// 
        /// # Returns
        /// ```
        /// Datapair<D>
        /// ```
        /// 
        /// # Example
        /// ```
        /// let datapair:Datapair<u8> = Datapair::new(String::from("key"),0_u8);
        /// ```
        /// 
        /// # Coution
        /// The type for the data has to be set.
        /// Wrong example:
        /// ```
        /// let datapair = Datapair::new(String::from("key"),0);
        /// ```
        /// This gives an error, because the type for the data is not clear.
        pub fn new(k:String,d:D) -> Self {
            Datapair {
                key:    k,
                data:   d
            }
        }
        /// changes the key
        /// 
        /// # Params
        /// ```
        /// k:String // new key
        /// ```
        /// 
        /// # Example
        /// ```
        /// // create datapair
        /// let mut datapair:Datapair<u8> = Datapair::new(String::from("key"),0_u8);
        /// 
        /// // set new key
        /// datapair.set_key(String::from("new"));
        /// ```
        pub fn set_key(&mut self,k:String) {
            self.key    =   k;
        }
        /// get the key
        /// 
        /// # Returns
        /// ```
        /// &String
        /// ```
        /// 
        /// # Example
        /// ```
        /// // create datapair
        /// let mut datapair:Datapair<u8> = Datapair::new(String::from("key"),0_u8);
        /// 
        /// // get key
        /// let key = datapair.get_key();
        /// ```
        pub fn get_key(&self) -> &String {
            &self.key
        }
        /// changes the data
        /// 
        /// # Params
        /// ```
        /// d:D // the new data
        /// ```
        /// 
        /// # Example
        /// ```
        /// // create datapair
        /// let mut datapair:Datapair<u8> = Datapair::new(String::from("key"),0_u8);
        /// 
        /// // set new data
        /// datapair.set_data(1_u8);
        /// ```
        pub fn set_data(&mut self,d:D) {
            self.data   =   d;
        }
        /// get the data
        /// 
        /// # Returns
        /// ```
        /// &D
        /// ```
        /// 
        /// # Example
        /// ```
        /// // create datapair
        /// let mut datapair:Datapair<u8> = Datapair::new(String::from("key"),0_u8);
        /// 
        /// // get the data
        /// let data = datapair.get_data();
        /// ```
        pub fn get_data(&self) -> &D {
            &self.data
        }
    }
    impl<D: Clone> Clone for Datapair<D> {
        fn clone(&self) -> Self {
            Datapair::new(self.key.clone(),self.data.clone())
        }
    }
    impl<D: std::fmt::Display> std::fmt::Display for Datapair<D> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let open = String::from("\u{007B}");
            let close = String::from("\u{007D}");
            write!(f, "{}{}, {}{}", open, self.key,self.data, close)
        }
    }
    impl<D: PartialEq> PartialEq for Datapair<D> {
        fn eq(&self, other: &Self) -> bool {
            if self.get_key() == other.get_key() {
                if self.get_data() == other.get_data() {
                    return true;
                }
            }
            return false;
        }
        fn ne(&self, other: &Self) -> bool {
            if self.get_key() == other.get_key() {
                if self.get_data() == other.get_data() {
                    return false;
                }
            }
            return true;
        }
    }
}