package com.yisui;

import org.springframework.context.ApplicationContext;
import org.springframework.context.support.ClassPathXmlApplicationContext;

public class App {

    public static void main(String[] args) {

        {
            System.out.println("test case 1");
            ApplicationContext context = new ClassPathXmlApplicationContext(
                    "applicationContext.xml");
            HelloWorld obj = (HelloWorld) context.getBean("helloBean");
            obj.setName("demo1");
            obj.SayHi();
            System.out.println("test case 1 end");
        }

        {
            System.out.println("test case 2");
            ApplicationContext context = new ClassPathXmlApplicationContext(
                    "applicationContext.xml");
            HelloWorld obj = (HelloWorld) context.getBean("helloBean1");
            obj.setName("demoBean");
            obj.SayHi();

            HelloWorld obj1 = (HelloWorld) context.getBean("helloBean1");
            obj1.setName("demoBean2");
            obj1.SayHi();
            obj1.SayMessage();
            System.out.println("test case 2 end");
        }
    }
}
