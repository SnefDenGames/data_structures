/// anything for the dictonary data structure
pub mod dictonary {
    /// a list of `Datapair`s
    pub struct Dictonary<D> {
        /// the `Datapair`s
        pairs:Vec<Datapair<D>>
    }
    impl<D> Dictonary<D> {
        /// create a new `Dictonary` for D
        /// 
        /// # Return
        /// ```
        /// Dictonary<D>
        /// ```
        /// 
        /// # Examples
        /// ```
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// ```
        /// 
        /// # Coution
        /// The type for the data has to be set.
        /// Wrong example:
        /// ```
        /// let dictonary = Dictonary::new();
        /// ```
        /// This gives an error, because the type for the data is not clear.
        pub fn new() -> Self {
            Dictonary {
                pairs:Vec::new()
            }
        }
        /// add a pair to the `Dictonary`
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// d:D // the data
        /// ```
        /// 
        /// # Example
        /// ```
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// 
        /// // add pair
        /// dictonary.add_pair("one".to_string(),1_u8);
        /// ```
        pub fn add_pair(&mut self,k:String,d:D) {
            self.pairs.push(Datapair::new(k,d));
        }
        /// removes a pair from the `Dictonary`
        /// 
        /// # Params
        /// ```
        /// k:String // the key
        /// ```
        /// 
        /// # Example
        /// ```
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),1_u8);
        /// 
        /// // remove pair
        /// dictonary.remove_pair("one");
        /// ```
        pub fn remove_pair(&mut self,k:String) {
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
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),0_u8);
        /// 
        /// // change data
        /// dictonary.change_data("one".to_string(),1_u8);
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
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),0_u8);
        /// 
        /// // change key
        /// dictonary.change_key("one".to_string(),"zero".to_string());
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
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),1_u8);
        /// 
        /// // get pair
        /// let pair = ("one",dictonary.get_data("one".to_string()));
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
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),1_u8);
        /// 
        /// // get data
        /// let data = dictonary.get_data("Laila".to_string());
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
        /// clears the `Dictonary`
        /// 
        /// # Example
        /// ```
        /// // creating dictonary
        /// let mut dictonary:Dictonary<u8> = Dictonary::new();
        /// dictonary.add_pair("one".to_string(),1_u8);
        /// dictonary.add_pair("two".to_string(),2_u8);
        /// 
        /// // clear all pairs
        /// dictonary.clear();
        /// ```
        pub fn clear(&mut self) {
            self.pairs  =   Vec::new();
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
        /// Dictonary<D>
        /// ```
        /// 
        /// # Example
        /// ```
        /// let datapair:Datapair<u8> = Datapair::new("key".to_string(),0_u8);
        /// ```
        /// 
        /// # Coution
        /// The type for the data has to be set.
        /// Wrong example:
        /// ```
        /// let datapair = Datapair::new("key".to_string(),0);
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
        /// let mut datapair:Datapair<u8> = Datapair::new("key".to_string(),0_u8);
        /// 
        /// // set new key
        /// datapair.set_key("new".to_string());
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
        /// let mut datapair:Datapair<u8> = Datapair::new("key".to_string(),0_u8);
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
        /// let mut datapair:Datapair<u8> = Datapair::new("key".to_string(),0_u8);
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
        /// let mut datapair:Datapair<u8> = Datapair::new("key".to_string(),0_u8);
        /// 
        /// // get the data
        /// let data = datapair.get_data();
        /// ```
        pub fn get_data(&self) -> &D {
            &self.data
        }
    }
}