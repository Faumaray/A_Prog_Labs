window.onload = function() {
   var search_by_name = document.getElementById("input_for_search_by_name");
   var button_by_name = document.getElementById("button_for_search_by_name");
   var form_by_name = document.getElementById("form_by_name");
   var search_by_id = document.getElementById("input_for_search_by_id");
   var button_by_id = document.getElementById("button_for_search_by_id");
   var form_by_id = document.getElementById("form_by_id");
   var delete_buttons = document.getElementsByClassName("delete");

   for (var i = 0; i < delete_buttons.length; i++) {
      delete_buttons[i].addEventListener("click", function() {
         deleteRowTable(this); 
      }, false);
    }
    
   search_by_name.onfocus  = function() {
      search_by_name.setAttribute("placeholder", "");
   }
   search_by_name.onblur = function () {
      search_by_name.setAttribute("placeholder", "Например, Иванов");
   }
   
   form_by_id.onsubmit = function () {
      if (isNaN(parseInt(search_by_id.value)))
      {
         alert("Need a Number!");
         return false;
      }
   }
   form_by_name.onsubmit = function () {
      var name = search_by_name.value;
      location.replace("create.html?" + name + "");
      return false;
   }
   deleteRowTable = function(ell) {
      if (confirm("Delete row?"))
      {
         row = ell.closest("tr"); // tr element (ваша строчка)
         row.parentElement.removeChild(row); // удаляем всю строку
      }
   }
}
