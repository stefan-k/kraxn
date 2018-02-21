var data = [4, 8, 15, 16, 23, 42];

var width = 420,
  bar_height = 20;

var x = d3.scale
  .linear()
  .domain([0, d3.max(data)])
  .range([0, width]);

var chart = d3
  .select(".chart")
  .attr("width", width)
  .attr("height", bar_height * data.length);

var bar = chart
  .selectAll("g")
  .data(data)
  .enter()
  .append("g")
  .attr("transform", function(d, i) {
    return "translate(0," + i * bar_height + ")";
  });

bar
  .append("rect")
  .attr("width", x)
  .attr("height", bar_height - 1);

bar
  .append("text")
  .attr("x", function(d) {
    return x(d) - 3;
  })
  .attr("y", bar_height / 2)
  .attr("dy", ".35em")
  .text(function(d) {
    return d;
  });

// var x = d3.scale
//   .linear()
//   .domain([0, d3.max(data)])
//   .range([0, 420]);
//
// d3
//   .select(".chart")
//   .selectAll("div")
//   .data(data)
//   .enter()
//   .append("div")
//   .style("width", function(d) {
//     return x(d) + "px";
//   })
//   .text(function(d) {
//     return d;
//   });
