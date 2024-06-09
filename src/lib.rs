// topology
pub mod edge2vtx;
pub mod elem2elem;
pub mod tri2vtx;
pub mod unify_index;
pub mod vtx2elem;
pub mod vtx2vtx;

// functions specific to type of mesh
pub mod map_idx;
pub mod polyline;
pub mod polyline3;
pub mod polyloop;
pub mod polyloop2;
pub mod polyloop3;
pub mod quadmesh;
pub mod trimesh2;
pub mod trimesh3;
pub mod trimesh3_primitive;
pub mod trimesh3_search_bruteforce;
pub mod vtx2xyz;

// misc functions general to mesh type
pub mod dijkstra;
pub mod elem2center;
pub mod elem2group;
pub mod extract;
pub mod sampling;
pub mod transform;
pub mod unindex;

// io
pub mod io_nas;
pub mod io_obj;
pub mod io_off;
pub mod io_vtk;

// search
pub mod bvh;
pub mod bvh3;
pub mod bvh3_topology_morton;
pub mod bvh3_topology_topdown;
pub mod kdtree2;

// self intersection
pub mod io_svg;
pub mod polygon_mesh;
pub mod trimesh;
pub mod trimesh2_dynamic;
pub mod trimesh3_intersection;
pub mod trimesh3_intersection_time;
pub mod trimesh3_move_avoid_intersection;
pub mod trimesh3_proximity;
pub mod trimesh_topology;
pub mod uniform_mesh;
pub mod voronoi2;
pub mod vtx2vec;
