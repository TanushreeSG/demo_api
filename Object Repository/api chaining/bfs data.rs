<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>bfs data</name>
   <tag></tag>
   <elementGuidId>b7c8df97-fbfa-4cb7-8b84-3f14a74123cf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA

</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MTM0MzQ4NzksIm9yZ0lkIjoxLCJpYXQiOjE3MDU2NTg4NzksImF1ZCI6IiIsImlzcyI6IiIsInN1YiI6IiJ9.brcKQYrzObA6tDod_IpN5Qu6viuzr-7rk5KmMUtlefPTBYQzKYpeBT5-Zx2JCRAiIeG6PIMvY4Hr1X9-L0FPOA

</value>
      <webElementGuid>1e67c82f-b96b-4006-a7c1-ad82e6399c25</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>169c51b5-3492-438a-87cb-8349cc50b912</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/doc/bfsData?name=district&amp;parent=Karnataka</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.devURL</defaultValue>
      <description></description>
      <id>937daa88-68c0-43a8-a14d-b0c2b8cc51a2</id>
      <masked>false</masked>
      <name>URL</name>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))


WS.verifyElementPropertyValue(response, 'status', 200)
WS.verifyElementPropertyValue(response, 'message', &quot;Data extracted successfully.&quot;)
WS.verifyElementPropertyValue(response, 'data[0].pid', 32)
WS.verifyElementPropertyValue(response, 'data[0].name', &quot;district&quot;)
WS.verifyElementPropertyValue(response, 'data[0].data[0].label', &quot;vijayapura&quot;)
WS.verifyElementPropertyValue(response, 'data[0].data[0].parent', &quot;Karnataka&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
