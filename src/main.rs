mod extract;

fn main() {
    let _a = "\\\\Desktop-22kf9ma\\an\\Anime";
    let _b = "\\\\Desktop-22kf9ma\\an\\Anime not";
    extract::iter_over_all_files(_a);
    extract::iter_over_all_files(_b);
}
