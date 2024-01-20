use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{ManyArgs, Opt},
        options::{CircleOptions, DistanceOptions, GetNearestOptions, Index},
    },
    Command,
};

create_cmd!(
    /// Construct a circular line or polygon.
    ///
    /// A circle in RethinkDB is a polygon or line approximating a circle of
    /// a given radius around a given center, consisting of a specified number
    /// of vertices (default 32).
    ///
    /// The center may be specified either by two floating point numbers,
    /// the latitude (−90 to 90) and longitude (−180 to 180) of the point
    /// on a perfect sphere (see Geospatial support for more information
    /// on ReQL’s coordinate system), or by a point object. The radius is
    /// a floating point number whose units are meters by default, although
    /// that may be changed with the `unit` argument.
    ///
    /// ## Example
    /// Define a circle.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("geo").insert(rjson!({
    ///     "id": 300,
    ///     "name": "Hayes Valley",
    ///     "neighborhood": r.circle(r.args(([-122.423246,37.779388], 1000)))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    /// - [point](Self::point)
    /// - [distance](Self::distance)
    only_root,
    circle(point_radius: ManyArgs<CircleOptions>)
);

create_cmd!(
    /// Compute the distance between a point and another geometry object.
    ///
    /// At least one of the geometry objects specified must be a point.
    ///
    /// If one of the objects is a polygon or a line, the point will be
    /// projected onto the line or polygon assuming a perfect sphere model
    /// before the distance is computed (using the model specified with
    /// `geoSystem`). As a consequence, if the polygon or line is extremely
    /// large compared to Earth’s radius and the distance is being computed
    /// with the default WGS84 model, the results of `distance` should be
    /// considered approximate due to the deviation between the ellipsoid
    /// and spherical models.
    ///
    /// ## Example
    /// Compute the distance between two points on the Earth in kilometers.
    ///
    /// ```
    /// # use unreql::cmd::options::DistanceOptions;
    /// # unreql::example(|r, conn| {
    /// let point1 = r.point(-122.423246, 37.779388);
    /// let point2 = r.point(-117.220406, 32.719464);
    /// let opts = DistanceOptions::new().unit("km".into());
    /// r.distance(point1, point2, opts).run(conn)
    /// // Result: 734.1252496021841
    /// # })
    /// ```
    only_root,
    distance(point1: Serialize, point2: Serialize, opts: Opt<DistanceOptions>)
    only_command,
    distance(point: Serialize, opts: Opt<DistanceOptions>)
);

create_cmd!(
    /// Convert a Line object into a Polygon object.
    ///
    /// If the last point does not specify the same coordinates as the
    /// first point, polygon will close the polygon by connecting them.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90) of vertices are
    /// plotted on a perfect sphere. See Geospatial support for more
    /// information on ReQL’s coordinate system.
    ///
    /// If the last point does not specify the same coordinates as the first
    /// point, polygon will close the polygon by connecting them. You cannot
    /// directly construct a polygon with holes in it using polygon, but you
    /// can use polygonSub to use a second polygon within the interior of the
    /// first to define a hole.
    ///
    /// ## Example
    /// Create a line object and then convert it to a polygon.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # use unreql::cmd::options::UpdateOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("geo").insert(rjson!({
    ///     "id": 201,
    ///     "rectangle": r.line(r.args([
    ///         [-122.423246,37.779388],
    ///         [-122.423246,37.329898],
    ///         [-121.886420,37.329898],
    ///         [-121.886420,37.779388]
    ///     ]))
    /// })).run(conn)
    /// # });
    ///
    /// # unreql::example(|r, conn| {
    /// r.table("geo").get(201).update(r.with_opt(
    ///     rjson!({
    ///         "rectangle": r.row().g("rectangle").fill()
    ///     }),
    ///     UpdateOptions::new().non_atomic(true)
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [line](Self::line)
    /// - [polygon](Self::polygon)
    only_command,
    fill
);

create_cmd!(
    /// Convert a [GeoJSON](http://geojson.org/) object to a ReQL geometry object.
    ///
    /// RethinkDB only allows conversion of GeoJSON objects which have ReQL
    /// equivalents: Point, LineString, and Polygon. MultiPoint,
    /// MultiLineString, and MultiPolygon are not supported. (You could,
    /// however, store multiple points, lines and polygons in an array and
    /// use a geospatial multi index with them.)
    ///
    /// Only longitude/latitude coordinates are supported. GeoJSON objects
    /// that use Cartesian coordinates, specify an altitude, or specify their
    /// own coordinate reference system will be rejected.
    ///
    /// ## Example
    /// Convert a GeoJSON object to a ReQL geometry object.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// let geojson = rjson!({
    ///     "type": "Point",
    ///     "coordinates": [ -122.423246, 37.779388 ]
    /// });
    /// r.table("geo").insert(rjson!({
    ///     "id": "sfo",
    ///     "name": "San Francisco",
    ///     "location": r.geojson(geojson),
    /// })).run(conn)
    /// # })
    /// ```
    only_root,
    geojson(geojson: Serialize)
);

create_cmd!(
    /// Convert a ReQL geometry object to a [GeoJSON](http://geojson.org/) object.
    ///
    /// ## Example
    /// Convert a ReQL geometry object to a GeoJSON object.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("geo").get("sfo").g("location").to_geojson().run(conn)
    /// # })
    /// ```
    ///
    /// Result
    ///
    /// ```json
    /// {
    ///     "type": "Point",
    ///     "coordinates": [ -122.423246, 37.779388 ]
    /// }
    /// ```
    only_command,
    to_geojson
);

create_cmd!(
    /// Get all documents where the given geometry object intersects
    /// the geometry object of the requested geospatial index.
    ///
    /// The `index` argument is mandatory. This command returns the same
    /// results as `table.filter(r.row().g("index").intersects(geometry))`.
    /// The total number of results is limited to the array size limit
    /// which defaults to 100,000, but can be changed with the `arrayLimit`
    /// option to `run`.
    ///
    /// ## Example
    /// Which of the locations in a list of parks intersect circle1?
    ///
    /// ```
    /// # use unreql::cmd::options::CircleOptions;
    /// # unreql::example(|r, conn| {
    /// let unit = CircleOptions::new().unit("mi".into());
    /// let circle1 = r.circle(r.with_opt(r.args(([-117.220406,32.719464], 10)), unit));
    /// r.table("parks").get_intersecting(circle1, r.index("area")).run(conn)
    /// # })
    /// ```
    get_intersecting(geometry: Serialize, opts: Opt<Index>)
);

create_cmd!(
    /// Return a list of documents closest to a specified point based on
    /// a geospatial index, sorted in order of increasing distance.
    ///
    /// The `index` argument is mandatory.
    ///
    /// The return value will be an array of two-item objects with the keys
    /// dist and doc, set to the distance between the specified point and
    /// the document (in the units specified with unit, defaulting to meters)
    /// and the document itself, respectively. The array will be sorted by
    /// the values of dist.
    ///
    /// ## Example
    /// Return a list of the closest 25 enemy hideouts to the secret base.
    ///
    /// ```
    /// # use unreql::cmd::options::GetNearestOptions;
    /// # unreql::example(|r, conn| {
    /// let secret_base = r.point(-122.422876,37.777128);
    /// let opts = GetNearestOptions::new()
    ///     .index("area".into())
    ///     .max_results(25);
    /// r.table("hideouts").get_nearest(secret_base, opts).run(conn)
    /// # })
    /// ```
    ///
    /// *Note*: If you wish to find all points within a certain radius of
    /// another point, it’s often faster to use getIntersecting with circle,
    /// as long as the approximation of a circle that circle generates is
    /// sufficient.
    get_nearest(point: Serialize, opts: Opt<GetNearestOptions>)
);

create_cmd!(
    /// Tests whether a geometry object is completely contained within another.
    ///
    /// When applied to a sequence of geometry objects, includes acts as
    /// a filter, returning a sequence of objects from the sequence that
    /// include the argument.
    ///
    /// ## Example
    /// Is point2 included within a 2000-meter circle around point1?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let point1 = r.point(-122.423246, 37.779388);
    /// let point2 = r.point(-117.220406, 32.719464);
    /// r.circle(r.args((point1, 2000))).includes(point2).run(conn)
    /// // Result: true
    /// # })
    /// ```
    includes(geometry: Serialize)
);

create_cmd!(
    /// Tests whether two geometry objects intersect with one another.
    ///
    /// When applied to a sequence of geometry objects, intersects acts as
    /// a filter, returning a sequence of objects from the sequence that
    /// intersect with the argument.
    ///
    /// ## Example
    /// Is point2 within a 2000-meter circle around point1?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let point1 = r.point(-122.423246, 37.779388);
    /// let point2 = r.point(-117.220406, 32.719464);
    /// r.circle(r.args((point1, 2000))).intersects(point2).run(conn)
    /// // Result: true
    /// # })
    /// ```
    only_root,
    intersects(sequence_or_geometry: Serialize, geometry: Serialize),
    only_command,
    intersects(geometry: Serialize),
);

create_cmd!(
    /// Construct a geometry object of type Line.
    ///
    /// The line can be specified in one of two ways:
    ///
    /// - Two or more two-item arrays, specifying latitude and longitude
    ///   numbers of the line’s vertices;
    /// - Two or more Point objects specifying the line’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90) of vertices are
    /// plotted on a perfect sphere. See Geospatial support for more
    /// information on ReQL’s coordinate system.
    ///
    /// ## Example
    /// Define a line.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("geo").insert(rjson!({
    ///     "id": 101,
    ///     "route": r.line(r.args([[-122.423246,37.779388], [-121.886420,37.329898]]))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Define a line using an array of points.
    /// You can use the args command to pass an array of Point objects (or latitude-longitude pairs) to line.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// let route = [
    ///     [-122.423246, 37.779388],
    ///     [-121.886420, 37.329898],
    /// ];
    /// r.table("geo").insert(rjson!({
    ///     "id": 102,
    ///     "route": r.line(r.args(route)),
    /// })).run(conn)
    /// # })
    /// ```
    only_root,
    line(points: ManyArgs<()>)
);

create_cmd!(
    /// ruct a geometry object of type Point.
    ///
    /// The point is specified by two floating point numbers, the longitude
    /// (−180 to 180) and latitude (−90 to 90) of the point on a perfect
    /// sphere. See [Geospatial support](https://rethinkdb.com/docs/geo-support/javascript/) for more information on ReQL’s
    /// coordinate system.
    ///
    /// ## Example
    /// Define a point.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("geo").insert(rjson!({
    ///     "id": 1,
    ///     "name": "San Francisco",
    ///     "location": r.point(-122.423246, 37.779388)
    /// })).run(conn)
    /// # })
    /// ```
    only_root,
    point(longitude: Serialize, latitude: Serialize)
);

create_cmd!(
    /// Construct a geometry object of type Polygon.
    ///
    /// The Polygon can be specified in one of two ways:
    ///
    /// - Three or more two-item arrays, specifying latitude and longitude
    ///   numbers of the polygon’s vertices;
    /// - Three or more Point objects specifying the polygon’s vertices.
    ///
    /// Longitude (−180 to 180) and latitude (−90 to 90) of vertices are
    /// plotted on a perfect sphere. See Geospatial support for more
    /// information on ReQL’s coordinate system.
    ///
    /// If the last point does not specify the same coordinates as the
    /// first point, polygon will close the polygon by connecting them.
    /// You cannot directly construct a polygon with holes in it using
    /// polygon, but you can use polygonSub to use a second polygon
    /// within the interior of the first to define a hole.
    ///
    /// ## Example
    /// Define a polygon.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("geo").insert(rjson!({
    ///     "id": 101,
    ///     "rectangle": r.polygon(r.args([
    ///         [-122.423246,37.779388],
    ///         [-122.423246,37.329898],
    ///         [-121.886420,37.329898],
    ///         [-121.886420,37.779388]
    ///     ]))
    /// })).run(conn)
    /// # })
    /// ```
    only_root,
    polygon(points: ManyArgs<()>)
);

create_cmd!(
    /// Use `polygon2` to “punch out” a hole in `polygon1`.
    ///
    /// `polygon2` must be completely contained within `polygon1` and must
    /// have no holes itself (it must not be the output of `polygon_sub` itself).
    ///
    /// ## Example
    /// Define a polygon with a hole punched in it.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// let outer_polygon = r.polygon(r.args([
    ///     [-122.4,37.7],
    ///     [-122.4,37.3],
    ///     [-121.8,37.3],
    ///     [-121.8,37.7]
    /// ]));
    /// let inner_polygon = r.polygon(r.args([
    ///     [-122.3,37.4],
    ///     [-122.3,37.6],
    ///     [-122.0,37.6],
    ///     [-122.0,37.4]
    /// ]));
    /// outer_polygon.polygon_sub(inner_polygon).run(conn)
    /// # })
    /// ```
    polygon_sub(polygon2: Serialize)
);
