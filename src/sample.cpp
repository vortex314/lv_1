#include "../../../lv_examples.h"
#include "L_Global.h"

#define TABLE_COL_NUMBER 5
#define TABLE_ROW_NUMBER 16

  static uint16_t row[2] = {0};
  static uint16_t col[2] = {0};
  static bool status[TABLE_ROW_NUMBER];  
  
static void test_event_handler(lv_obj_t * obj, lv_event_t event)
{
  static uint16_t row_temp;
  static uint16_t col_temp;
  static lv_res_t r;
  
  if(event == LV_EVENT_PRESSED)
  {
     r = lv_table_get_pressed_cell(obj, &row_temp, &col_temp);
     sysprintf("table_key_event: %d:status: %d ,row: %d,col: %d\n",event,r,row[0],col[0]);

  }
  else if(event == LV_EVENT_SHORT_CLICKED)
  {
     if(r == LV_RES_INV) return; 
     row[1] = row[0];
     col[1] = col[0];
     row[0] = row_temp;
     col[0] = col_temp;
     
     status[row[0]] = !status[row[0]];
     if(status[row[0]] == true)
       lv_table_set_cell_value(obj, row[0], 0, LV_SYMBOL_OK);
     else
       lv_table_set_cell_value(obj, row[0], 0, "");
    
     if(row[1] == row[0])
    {
        if(status[row[0]] == true)
          for(uint32_t i = 0; i < TABLE_COL_NUMBER;i++)
            lv_table_set_cell_type(obj, row[0], i, LV_TABLE_PART_CELL2);
        else
           for(uint32_t i = 0; i < TABLE_COL_NUMBER;i++)
            lv_table_set_cell_type(obj, row[0], i, LV_TABLE_PART_CELL1);
    }else
    {
      for(uint32_t i =0; i < TABLE_COL_NUMBER;i++)
      {
        lv_table_set_cell_type(obj, row[1], i, LV_TABLE_PART_CELL1);
        lv_table_set_cell_type(obj, row[0], i, LV_TABLE_PART_CELL2);
      }
    }
  }  
}

void lv_ex_table_1(void)
{
    memset(status,0x0,sizeof(status));
    static lv_style_t style;
    lv_style_init(&style);
    lv_style_set_pad_top(&style, LV_STATE_DEFAULT, 0); 
    lv_style_set_pad_bottom(&style, LV_STATE_DEFAULT, 0);
    lv_style_set_pad_left(&style, LV_STATE_DEFAULT, 0);
    lv_style_set_pad_right(&style, LV_STATE_DEFAULT, 0);
    lv_style_set_line_opa(&style, LV_STATE_DEFAULT, 0);
    lv_style_set_pad_inner(&style, LV_STATE_DEFAULT, 0);
    lv_style_set_radius(&style, LV_STATE_DEFAULT, 0);
    
    lv_obj_t *  cont = lv_cont_create(lv_scr_act(), NULL);
    lv_obj_set_auto_realign(cont, true); 
    lv_obj_align_origo(cont, NULL, LV_ALIGN_CENTER, 0, 0); 
    lv_cont_set_fit(cont, LV_FIT_TIGHT);
    lv_cont_set_layout(cont, LV_LAYOUT_COLUMN_MID);
    lv_obj_add_style(cont, LV_OBJ_PART_MAIN, &style);
    
    static lv_style_t style1;
    lv_style_init(&style1);
    lv_style_set_pad_top(   &style1, LV_STATE_DEFAULT, 16); 
    lv_style_set_pad_bottom(&style1, LV_STATE_DEFAULT, 16);
    lv_style_set_pad_left(  &style1, LV_STATE_DEFAULT, 16);
    lv_style_set_pad_right( &style1, LV_STATE_DEFAULT, 16);
    lv_style_set_pad_inner( &style1, LV_STATE_DEFAULT, 2);
    lv_style_set_text_font (&style1, LV_STATE_DEFAULT,&lv_font_montserrat_16);
  
    static lv_style_t style2;
    lv_style_init(&style2);
    lv_style_copy(&style2,&style1);
    lv_style_set_bg_color(    &style2, LV_STATE_DEFAULT, LV_COLOR_MAKE(0, 255, 0));
    lv_style_set_bg_color(   &style2, LV_STATE_PRESSED, LV_COLOR_MAKE(0, 255, 0));
    lv_style_set_text_color(  &style2, LV_STATE_DEFAULT, LV_COLOR_MAKE(255, 0, 255));
    lv_style_set_line_color(  &style2, LV_STATE_DEFAULT, LV_COLOR_MAKE(255, 0, 0));
    lv_style_set_border_color(&style2, LV_STATE_DEFAULT, LV_COLOR_MAKE(0, 0, 255));
    lv_style_set_border_opa(  &style2, LV_STATE_DEFAULT, 100);
    
    /*表头部分*/
    lv_obj_t * table = lv_table_create(cont, NULL);
    lv_table_set_col_cnt(table, TABLE_COL_NUMBER);
    lv_table_set_row_cnt(table, 1);
    lv_obj_align(table, NULL, LV_ALIGN_IN_TOP_LEFT, 0, 0);
    static lv_style_t style_tabh;
    lv_style_init(&style_tabh);
    lv_style_copy(&style_tabh,&style);
    lv_style_set_pad_right(&style_tabh, LV_STATE_DEFAULT, 50);
    lv_style_set_margin_bottom(&style_tabh, LV_STATE_DEFAULT, 0);
    lv_style_set_margin_top(&style_tabh, LV_STATE_DEFAULT, 5);
    lv_style_set_border_opa(&style_tabh, LV_STATE_DEFAULT, 0);
    lv_style_set_bg_color(  &style_tabh, LV_STATE_DEFAULT, LV_COLOR_MAKE(200, 255, 200));
    lv_obj_add_style(table, LV_OBJ_PART_MAIN, &style_tabh);
    lv_obj_add_style(table, LV_TABLE_PART_CELL1, &style1);  
    
    lv_table_set_cell_align(table, 0, 0, LV_LABEL_ALIGN_CENTER);
    lv_table_set_cell_align(table, 0, 1, LV_LABEL_ALIGN_CENTER);
    lv_table_set_cell_align(table, 0, 2, LV_LABEL_ALIGN_CENTER);
    lv_table_set_cell_align(table, 0, 3, LV_LABEL_ALIGN_CENTER);
    lv_table_set_cell_align(table, 0, 4, LV_LABEL_ALIGN_CENTER);
    
    lv_table_set_cell_type(table, 0, 0, LV_TABLE_PART_CELL1);
    lv_table_set_cell_type(table, 0, 1, LV_TABLE_PART_CELL1);
    lv_table_set_cell_type(table, 0, 2, LV_TABLE_PART_CELL1);
    lv_table_set_cell_type(table, 0, 3, LV_TABLE_PART_CELL1);
    lv_table_set_cell_type(table, 0, 4, LV_TABLE_PART_CELL1);

    
    lv_table_set_cell_value(table, 0, 0, "Select");
    lv_table_set_cell_value(table, 0, 1, "FileName");
    lv_table_set_cell_value(table, 0, 2, "FileSize");
    lv_table_set_cell_value(table, 0, 3, "Picks");
    lv_table_set_cell_value(table, 0, 4, "date");
    lv_table_set_col_width(table, 0, 100);
    lv_table_set_col_width(table, 0, 200);
    lv_table_set_col_width(table, 1, 200);
    lv_table_set_col_width(table, 2, 200);
    lv_table_set_col_width(table, 3, 200);
    /*表体部分*/
    lv_obj_t * page = lv_page_create(cont, NULL);
    lv_obj_set_size(page, 950, 500);
    lv_obj_align(page, table, LV_ALIGN_OUT_BOTTOM_LEFT, 0, 0);
     static lv_style_t style_page;
    lv_style_init(&style_page);
    lv_style_copy(&style_page,&style);
    lv_style_set_border_opa(&style_page, LV_STATE_DEFAULT, 0);
    lv_obj_add_style(page, LV_OBJ_PART_MAIN, &style_page);
    
   
    lv_obj_t * table1 = lv_table_create(page, NULL);
    lv_table_set_col_cnt(table1, TABLE_COL_NUMBER);
    lv_table_set_row_cnt(table1, TABLE_ROW_NUMBER);
    lv_obj_align(table1, page, LV_ALIGN_IN_TOP_LEFT, 0, 0);
    lv_obj_set_drag_parent(table1, true);  
     static lv_style_t style_tabh1;
    lv_style_init(&style_tabh1);
    lv_style_copy(&style_tabh1,&style);
    lv_style_set_margin_bottom(&style_tabh1, LV_STATE_DEFAULT, 5);
    lv_style_set_margin_top(&style_tabh1, LV_STATE_DEFAULT, 0);
    lv_style_set_border_opa(&style_tabh1, LV_STATE_DEFAULT, 0);
    
    lv_obj_add_style(table1, LV_OBJ_PART_MAIN, &style_tabh1);
    lv_obj_add_style(table1, LV_TABLE_PART_CELL1, &style1);    
    lv_obj_add_style(table1, LV_TABLE_PART_CELL2, &style2);
    
    lv_obj_set_event_cb(table1, test_event_handler);
    for(uint32_t i = 0; i< TABLE_ROW_NUMBER; i++)
    {
      lv_table_set_cell_value(table1, i, 0, "");
      lv_table_set_cell_value(table1, i, 1, "Price.jc5");
      lv_table_set_cell_value(table1, i, 2, "32768");
      lv_table_set_cell_value(table1, i, 3, "45");
      lv_table_set_cell_value(table1, i, 4, "2020.08.01");
      lv_table_set_cell_type(table1, i, 0, LV_TABLE_PART_CELL1);
      lv_table_set_cell_type(table1, i, 1, LV_TABLE_PART_CELL1);
      lv_table_set_cell_type(table1, i, 2, LV_TABLE_PART_CELL1);
      lv_table_set_cell_type(table1, i, 3, LV_TABLE_PART_CELL1);
      lv_table_set_cell_type(table1, i, 4, LV_TABLE_PART_CELL1);
      lv_table_set_cell_align(table1, i, 0, LV_LABEL_ALIGN_CENTER);
      lv_table_set_cell_align(table1, i, 1, LV_LABEL_ALIGN_CENTER);
      lv_table_set_cell_align(table1, i, 2, LV_LABEL_ALIGN_CENTER);
      lv_table_set_cell_align(table1, i, 3, LV_LABEL_ALIGN_CENTER);
      lv_table_set_cell_align(table1, i, 4, LV_LABEL_ALIGN_CENTER);
        
    } 
    lv_table_set_col_width(table1, 0, 100);
    lv_table_set_col_width(table1, 1, 200);
    lv_table_set_col_width(table1, 2, 200);
    lv_table_set_col_width(table1, 3, 200);
    lv_table_set_col_width(table1, 4, 200);
}

