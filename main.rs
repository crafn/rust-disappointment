// rustc version: 0.12.0-pre-nightly 2014-07-18
// compilation: rustc main.rs
// running: main map1

// rust-disappointment: "using namespace std" on by default

mod math {
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }
    
    // rust-disappointment: tedious syntax for such a trivial thing
    impl<T: Add<T, T>> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
        fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
            Vec2{ x: self.x + rhs.x, y: self.y + rhs.y }
        }
    }
} // math

mod map {
    // rust-disappointment: leading :: necessary in this case?
    pub type TileVec= ::math::Vec2<i32>;
    // rust-disappointment: how much value does `::std::vec::` add in respect to `std::` ?
    type Tiles= ::std::vec::Vec<Tile>;
    
    enum TileType {
        // rust-disappointment: declaring None in enum will break range based
        // for loops:
        //   for i in range(0i, 10i) {}
        // -> error: mismatched types: expected `core::option::Option<int>`
        // but found `map::TileType` (expected enum core::option::Option but
        // found enum map::TileType)
        Non,
        Ground,
        Road,
    }
    
    pub struct Tile {
        // rust-disappointment: type is a keyword
        tpe: TileType,
    }
    
    pub struct Map {
        tiles: Tiles,
        size: TileVec,
    }
    
    impl Map {
        // rust-disappointment: method `getTileIndex` should have a snake
        // case name such as `get_tile_index`, #[warn(non_snake_case_functions)]
        // on by default
        fn get_tile_index(&self, p: TileVec) -> uint {
            (p.x + p.y*self.size.x) as uint
        }
        
        pub fn get_tile<'l>(&'l self, p: TileVec) -> &'l Tile {
            let i= self.get_tile_index(p);
            return &self.tiles[i];
        }
        
        // rust-disappointment: this is really bad: mutability can't be
        // parameterized so code has to be duplicated. This is even worse than
        // in C++ because functions can't be overloaded, but has to be named
        // differently, which makes adding parameterized mutability afterwards painful.
        pub fn get_mut_tile<'l>(&'l mut self, p: TileVec) -> &'l mut Tile {
            let i= self.get_tile_index(p);
            // rust-disappointment: having language constructs embedded in
            // function names smells like bad design
            return &mut self.tiles.as_mut_slice()[i];
        }
        
        pub fn new(s: TileVec) -> Map {
            let mut tiles= vec![];
            
            for _ in range(0, s.x*s.y) {
            	tiles.push(Tile{ tpe: Non });
            }
            
            // rust-disappointment: Passing initial values for fields use the same
            // syntax as type annotations - why not `=` like in `let x= 5i;` ?
            return Map{ tiles: tiles, size: s };
        }
        
        pub fn print(&self) -> () {
            // rust-delight: range-based for loops are good.
            for y in range(0, self.size.y) {
                for x in range (0, self.size.x) {
                    print!("{}",
                        tile_type_to_char(
                            self.get_tile(TileVec{x: x, y: y}).tpe
                        )
                    );
                }
                print!("\n");
            }
        }
    } // impl Map
    
    fn char_to_tile_type(ch: char) -> TileType {
        // rust-delight: match is good.
        match ch {
            '#' => Ground,
            '.' => Road,
            _ => Non,
        }
    }
    
    // For testing
    fn tile_type_to_char(t: TileType) -> char {
        match t {
            Non => ' ',
            Ground => '#',
            Road => '.',
        }
    }
    
    pub fn load(mapname: &str) -> Map {
        use std::io::BufferedReader;
        use std::io::File;
        
        let map_path= Path::new(mapname);
        let mut file= BufferedReader::new(File::open(&map_path));
        
        // rust-disappointment: Type of a variable can change after initialization!?
        //   let mut size= TileVec{x: 0, y: 0};
        //   size.x= 5u;
        //   let map= Map::new(size);
        // -> error: mismatched types: expected `Vec2<i32>` but found `Vec2<uint>`
        
        let mut size: TileVec = TileVec{x: 0, y: 0};
        let mut lines= vec![];
        for line_iter in file.lines() {
            let line= match line_iter { Ok(x) => x, Err(e) => fail!(e) };
            let len= line.len() as i32;
            
            // rust-disappointment: using {} is mandatory
            if len > size.x {
                size.x= len;
            }
            // rust-disappointment: no ++ operator
            size.y += 1;
            lines.push(line);
        }
        
        let mut map= Map::new(size);
        let mut p: TileVec = TileVec{x: 0, y: 0};
        for line in lines.iter() {
            for ch in line.as_slice().chars() {
                map.get_mut_tile(p).tpe= char_to_tile_type(ch);
                p.x += 1;
            }
            p.y += 1;
            p.x= 0;
        }
        return map;
    }
} // map

fn main(){
    let args= std::os::args();
    let map= map::load(args[1].as_slice());
    map.print();
}
