<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get scanned results</name>
   <tag></tag>
   <elementGuidId>3695d7c5-e515-483a-8393-3a01f675143f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA</value>
      <webElementGuid>5d02d035-2d67-4a3e-a794-eb6835a6f6cd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bb351590-99c7-4364-9bc9-b4bdcb999e19</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/doc/scan-results?queue_id=842</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.devURL</defaultValue>
      <description></description>
      <id>ff4bd9a0-5e79-4b15-9cf8-48a54ea16d28</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.queue_id</defaultValue>
      <description></description>
      <id>5816749a-692e-420e-8026-1ab5b9df6237</id>
      <masked>false</masked>
      <name>queue_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
def sluper = new JsonSlurper()
def result = sluper.parseText(response.getResponseBodyContent())

println(&quot;GET API Result is : &quot; + result)

return result</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
