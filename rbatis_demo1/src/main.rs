//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate tokio;

use chrono::{NaiveDateTime};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::utils::table_util::FatherChildRelationship;
use rbatis::Error;
//use crate::rbatis::rbatis_core::value::DateTimeNow;

#[crud_enable]
#[derive(Clone, Debug)]
pub struct ActivityOptions {
	pub id: Option<String>,
	pub activity_id: Option<String>,
	pub name: Option<String>,
	pub value: Option<String>,
}


/// may also write `CRUDTable` as `impl CRUDTable for BizActivity{}`
/// #[crud_enable( table_name:biz_activity)]
/// #[crud_enable(id_name:"id"|id_type:"String"|table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
#[crud_enable]
#[derive(Clone, Debug)]
pub struct BizActivity {
  pub id: Option<String>,
  pub name: Option<String>,
  pub pc_link: Option<String>,
  pub h5_link: Option<String>,
  pub pc_banner_img: Option<String>,
  pub h5_banner_img: Option<String>,
  pub sort: Option<String>,
  pub status: Option<i32>,
  pub remark: Option<String>,
  pub create_time: Option<NaiveDateTime>,
  pub version: Option<i32>,
  pub delete_flag: Option<i32>,
  pub parent_id: Option<String>,
  pub childs: Vec<BizActivity>,
  pub activity_options: Vec<ActivityOptions>,
}

// (optional) manually implement instead of using `derive(CRUDTable)`. This allows manually rewriting `table_name()` function and supports  code completion in IDE.
// use rbatis::crud::CRUDTable;
//impl CRUDTable for BizActivity {
//    type IdType = String;    
//    fn table_name()->String{
//        "biz_activity".to_string()
//    }
//    fn table_columns()->String{
//        "id,name,delete_flag".to_string()
//    }
//}

impl FatherChildRelationship for BizActivity {
		fn get_father_id(&self) -> Option<&Self::IdType> {
			self.parent_id.as_ref()
		}
		fn set_childs(&mut self, arg: Vec<Self>) {
			self.childs = arg;
		}
	}

#[tokio::main]
async fn main() {
	/// enable log crate to show sql logs
	fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
	/// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
	let rb = Rbatis::new();
	/// connect to database  
	rb.link("sqlite://:memory:").await.unwrap();
  
	rb.exec("", "CREATE TABLE `biz_activity` ( `id` varchar(50) NOT NULL DEFAULT '', `name` varchar(255) NOT NULL, `pc_link` varchar(255) DEFAULT NULL, `h5_link` varchar(255) DEFAULT NULL, `sort` varchar(255) NOT NULL, `status` int(11) NOT NULL, `version` int(11) NOT NULL, `remark` varchar(255) DEFAULT NULL, `create_time` datetime NOT NULL, `delete_flag` int(1) NOT NULL, `pc_banner_img` varchar(255) DEFAULT NULL, `h5_banner_img` varchar(255) DEFAULT NULL, parent_id varchar(50) DEFAULT NULL, PRIMARY KEY (`id`) );").await;

  /// customize connection pool parameters (optional)
// let mut opt =PoolOptions::new();
// opt.max_size=100;
// rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
  /// newly constructed wrapper sql logic
  let wrapper = rb.new_wrapper()
          .eq("id", 1)                    //sql:  id = 1
          .and()                          //sql:  and 
          .ne("id", 1)                    //sql:  id <> 1
          .in_array("id", &[1, 2, 3])     //sql:  id in (1,2,3)
          .not_in("id", &[1, 2, 3])       //sql:  id not in (1,2,3)
          .like("name", 1)                //sql:  name like 1
          .or()                           //sql:  or
          .not_like("name", "asdf")       //sql:  name not like 'asdf'
          .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")//sql:  create_time between '2020-01-01 00:00:00' and '2020-01-01 00:00:00'
          .group_by(&["id"])              //sql:  group by id
          .order_by(true, &["id", "name"])//sql:  group by id,name
          ;

//   let activity = BizActivity {
//     id: Some("12312".to_string()),
//     name: None,
//     remark: None,
//     
//     //create_time: Some(NaiveDateTime::new()),
//     create_time: Some(chrono::Local::now().naive_local()),
//     version: Some(1),
//     delete_flag: Some(1),
//     
//     // Why must these fields be given?
//     
//     h5_link: None,
//     pc_link: None,
//     pc_banner_img: None,
//     h5_banner_img: None,
//     sort: None,
//     status: None,
//     
//     
//   };
  
  let activity = BizActivity {
    id: Some("12312".to_string()),
    name: Some("MyName".to_string()),
    remark: Some("MyRemark".to_string()),
    
    //create_time: Some(NaiveDateTime::new()),
    create_time: Some(chrono::Local::now().naive_local()),
    version: Some(1),
    delete_flag: Some(1),
    
    // Why must these fields be given?
    
    h5_link: None,
    pc_link: None,
    pc_banner_img: None,
    h5_banner_img: None,
    sort: Some("MySort".to_string()),
    status: Some(1),
    parent_id: None,
    childs: vec![],
    activity_options: vec![],
    
  };
  
  let child_activity = BizActivity {
    id: Some("12313".to_string()),
    name: Some("MyChildName".to_string()),
    remark: Some("MyChildRemark".to_string()),
    
    //create_time: Some(NaiveDateTime::new()),
    create_time: Some(chrono::Local::now().naive_local()),
    version: Some(1),
    delete_flag: Some(1),
    
    // Why must these fields be given?
    
    h5_link: None,
    pc_link: None,
    pc_banner_img: None,
    h5_banner_img: None,
    sort: Some("MySort".to_string()),
    status: Some(1),
    parent_id: Some("12312".to_string()),
    childs: vec![],
    activity_options: vec![],
    
  };
  
  /// saving
  rb.save("", &activity).await;
  rb.save("", &child_activity).await;
  // Added for Father Child Relations
  
	let childs=vec![child_activity];
	let all_record = rbatis::make_table_field_map!(childs,id);
	activity.recursive_set_childs(&all_record);
	println!("{:#?}", activity);
	
	
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

  /// batch saving
  // rb.save_batch("", &vec![activity]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

  /// The query, Option wrapper, is None if the data is not found
  let result: Option<BizActivity> = rb.fetch_by_id("", &"1".to_string()).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

  /// query all
  //let result: Vec<BizActivity> = rb.list("").await.unwrap();
  let result: Vec<BizActivity> = rb.fetch_list("").await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1

  ///query by id vec
  //let result: Vec<BizActivity> = rb.list_by_ids("", &["1".to_string()]).await.unwrap();
  let result: Vec<BizActivity> = rb.fetch_list_by_ids("", &["1".to_string()]).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?) 

  ///query by wrapper
  let w = rb.new_wrapper().eq("id", "1");
  let r: Result<Option<BizActivity>, Error> = rb.fetch_by_wrapper("", &w).await;
//Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ? 

  ///delete
  rb.remove_by_id::<BizActivity>("", &"1".to_string()).await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

  ///delete batch
  rb.remove_batch_by_id::<BizActivity>("", &["1".to_string(), "2".to_string()]).await;
//Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  ) 

  ///update
  ///if version_lock plugin actived,update method will modify field 'version'= version + 1
  let mut activity = activity.clone();
  let w = rb.new_wrapper().eq("id", "12312");
  //rb.update_by_wrapper("", &mut activity, &w).await;
  rb.update_by_wrapper("", &mut activity, &w, false).await;
//Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ? 
}
