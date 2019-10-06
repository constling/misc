package com.yisui;

import org.springframework.beans.factory.InitializingBean;

public class HelloWorld implements InitializingBean {

    private String name = "susan";
    private String message = "message";

    protected void finalize() {

        System.out.println("HelloWorld finalize: " + name);
    }

    public void afterPropertiesSet() throws Exception  {
        System.out.println("Hello World afterpProperties");
    }

    public void SayHi() {
        System.out.println(name);
    }

    public void SayMessage() {
        System.out.println(message);
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getMessage() {
        return message;
    }

    public void setMessage(String message) {
        this.message = message;
    }
}

