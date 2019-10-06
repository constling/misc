package com.yisui.app;

import com.yisui.dao.StudentDao;
import org.springframework.context.ApplicationContext;
import org.springframework.context.support.ClassPathXmlApplicationContext;

import java.sql.DriverManager;
import java.sql.*;

public class App {

    public static void main(String[] args) {

        System.out.println("begin");

        {
            ApplicationContext context = new ClassPathXmlApplicationContext(
                    "applicationContext.xml");
            StudentDao dao = (StudentDao)context.getBean("StudentDao");
            dao.queryStudents();
        }
        {
            // first sample use code to jdbc
//            try {
//
//                String sql = "select * from student";
//                Connection con = null;
//                Statement stmt = null;
//                Class.forName("com.mysql.jdbc.Driver");
//                // 连接对象
//                con = DriverManager.getConnection("jdbc:mysql://localhost:3306/choumi?serverTimezone=GMT", "root", "123456");
//                // 执行命令对象
//                stmt =  con.createStatement();
//                // 执行
//                ResultSet rs = stmt.executeQuery(sql);
//                while (rs.next()) {
//                    int id = rs.getInt("id");
//                }
//                // 关闭
//                stmt.close();
//                con.close();
//            } catch (Exception e) {
//
//            }
        }

    }
}
