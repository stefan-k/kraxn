// var data = [4, 8, 15, 16, 23, 42];

var chart = d3.select(".chart");
var outer_width = +chart.attr("width");
var outer_height = +chart.attr("height");
var margin = { top: 20, right: 30, bottom: 30, left: 40 },
  width = outer_width - margin.left - margin.right,
  height = outer_height - margin.top - margin.bottom;

var g = chart
  .append("g")
  .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

var x = d3.scale.linear().rangeRound([0, width]);

var y = d3.scale.linear().rangeRound([height, 0]);

var line = d3.svg
  .line()
  .x(function(d) {
    return x(d.x);
  })
  .y(function(d) {
    return y(d.y);
  });

// var x_axis = d3.svg
//   .axis()
//   .scale(x)
//   .orient("bottom");
//
// var y_axis = d3.svg
//   .axis()
//   .scale(y)
//   .orient("left")
//   .ticks(10, "%");

// d3.tsv("js/frequency.tsv", type, function(error, data) {
d3.csv("data/1", type, function(error, data) {
  if (error) throw error;

  x.domain(
    data.map(function(d) {
      return d.x;
    })
  );

  y.domain([
    0,
    d3.max(data, function(d) {
      return d.y;
    })
  ]);

  g
    .append("g")
    .attr("transform", "translate(0," + height + ")")
    // .call(d3.axisBottom(x))
    .select(".domain")
    .remove();

  g
    .append("path")
    .datum(data)
    .attr("fill", "black")
    .attr("stroke", "steelblue")
    .attr("stroke-linejoin", "round")
    .attr("stroke-linecap", "round")
    .attr("stroke-width", 1.5)
    .attr("d", line);

  // chart
  //   .append("g")
  //   .attr("class", "x axis")
  //   .attr("transform", "translate(0," + height + ")")
  //   .call(x_axis);
  //
  // chart
  //   .append("g")
  //   .attr("class", "y axis")
  //   .call(y_axis)
  //   .append("text")
  //   .attr("transform", "rotate(-90)")
  //   .attr("y", 6)
  //   .attr("dy", ".71em")
  //   .style("text-anchor", "end")
  //   .text("Frequency");
  //
  // chart
  //   .selectAll(".bar")
  //   .data(data)
  //   .enter()
  //   .append("rect")
  //   .attr("class", "bar")
  //   .attr("x", function(d) {
  //     return x(d.letter);
  //   })
  //   .attr("y", function(d) {
  //     return y(d.frequency);
  //   })
  //   .attr("height", function(d) {
  //     return height - y(d.frequency);
  //   })
  //   .attr("width", x.rangeBand());
});

function type(d) {
  d.x = +d.x; // coerce to number
  d.y = +d.y; // coerce to number
  return d;
}
