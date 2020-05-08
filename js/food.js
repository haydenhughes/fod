// Handle adding and removing food items

function registerDelete() {
  $(".food-delete").click(function() {
    $("#" + $(this).data("target")).remove();
  });
};

$(document).ready(function() {
  var food_id = 1;

  $("#add-food").click(function() {
    var template = $($("template").prop("content")).find("#food-item").clone();
    var id = "food-item-" + food_id;
    template.attr("id", id);
    template.find(".input").attr("name", id);
    template.find(".food-delete").data("target", id);

    $("#food-items").append(template);

    food_id++;

    registerDelete();
  });
});
