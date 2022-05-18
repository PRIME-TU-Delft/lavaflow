use super::catmull_clark::{Vertex, Edge};
/// Gets a lava path of a specified length starting at a specfied point in the model.
///
/// # Arguments 
///
/// * `start` - The index of the first point in the lava flow.
/// * `length` - Amount of points to be added to the lava flow
/// * `vs` - List of all points of the model.
/// * `es` - List of neighbors indexes per point in  the model.
///
/// # Return
/// *  `Result<Vec< &'a Vertex>, String>` - Result of list of vertexes in lava path
///
pub fn get_lava_path<'a>(start: usize, length : usize, vs: &'a Vec<Vertex>, es: &'a Vec<Vec<usize>>) -> Result<Vec< &'a Vertex>, String>{

    let mut path = Vec::with_capacity(length);

    path.push(vs.get(start).ok_or(String::from("start point for lava does not exist in vertex list"))?);
    
    //index vertex pair of current point in parth
    let mut cur = (start, vs.get(start).ok_or(String::from("start point for lava does not exist in vertex list"))?);

    while (path.len() < length){

        //get neighbors
        //neighbor is (index, Vertex)
        let mut neighbors: Vec<(usize, &Vertex)> = Vec::new();

        for i in &es[cur.0] {
            neighbors.push((*i, vs.get(*i).ok_or(format!("lava_path: index {i} not found in vertex list"))?));
        }

        //per neighbor calculate gradient and find maximum
        let mut max = cur;
        let mut max_g = f32::MIN;
        for n in neighbors {   
            let new_g = gradient_between_points(cur.1, n.1 );
            if max_g < new_g {
                max = n;
                max_g = new_g;
            }
        }

        //add steepest neighbor to path
        path.push(max.1);
        //mark naighbor as next point
        cur = max;

    }
    Ok(path)
}

/// Gets a lava path of a specified length starting at a specfied point in the model.
///
/// # Arguments 
///
/// * `start` - The index of the first point in the lava flow.
/// * `length` - Amount of points to be added to the lava flow
/// * `vs` - List of all points of the model.
/// * `es` - List of neighbors indexes per point in  the model.
///
/// # Return
/// *  Result<Vec< &'a Vertex>, String>
///
fn gradient_between_points(from : &Vertex, to : &Vertex) -> f32 {
    //(sqr(from.x-to.x) + sqr(from.y-to.y) + sqr(from.z - to.z)).sqrt() * ((from.z - to.z) / (from.z - to.z))

    //gradient = diff in z
    //(from.z - to.z) 
    //gradient = z diff divided by length of edge
    (from.z - to.z) / (sqr(from.x-to.x) + sqr(from.y-to.y) + sqr(from.z - to.z)).sqrt()
}

fn sqr(a :f32) -> f32{
    a * a
}