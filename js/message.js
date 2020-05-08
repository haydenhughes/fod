// Handle message closing

$(document).ready(function() {
  $(".notification > .delete").click(function() {
    $(this).parent().remove()
  })
});
