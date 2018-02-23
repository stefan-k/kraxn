// var data = [4, 8, 15, 16, 23, 42];

var outer_width = 960;
var outer_height = 500;
var margin = { top: 20, right: 30, bottom: 30, left: 40 },
  width = outer_width - margin.left - margin.right,
  height = outer_height - margin.top - margin.bottom;

var x = d3.scale.ordinal().rangeRoundBands([0, width], 0.1);

var y = d3.scale.linear().range([height, 0]);

var x_axis = d3.svg
  .axis()
  .scale(x)
  .orient("bottom");

var y_axis = d3.svg
  .axis()
  .scale(y)
  .orient("left")
  .ticks(10, "%");

var chart = d3
  .select(".chart")
  .attr("width", outer_width)
  .attr("height", outer_height)
  .append("g")
  .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

d3.tsv("js/frequency.tsv", type, function(error, data) {
  if (error) throw error;
  x.domain(
    data.map(function(d) {
      return d.letter;
    })
  );
  y.domain([
    0,
    d3.max(data, function(d) {
      return d.frequency;
    })
  ]);

  chart
    .append("g")
    .attr("class", "x axis")
    .attr("transform", "translate(0," + height + ")")
    .call(x_axis);

  chart
    .append("g")
    .attr("class", "y axis")
    .call(y_axis)
    .append("text")
    .attr("transform", "rotate(-90)")
    .attr("y", 6)
    .attr("dy", ".71em")
    .style("text-anchor", "end")
    .text("Frequency");

  chart
    .selectAll(".bar")
    .data(data)
    .enter()
    .append("rect")
    .attr("class", "bar")
    .attr("x", function(d) {
      return x(d.letter);
    })
    .attr("y", function(d) {
      return y(d.frequency);
    })
    .attr("height", function(d) {
      return height - y(d.frequency);
    })
    .attr("width", x.rangeBand());
});

function type(d) {
  d.frequency = +d.frequency; // coerce to number
  return d;
}
