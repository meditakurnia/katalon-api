<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Posts</name>
   <tag></tag>
   <elementGuidId>7ed8e139-d0b4-4518-bf10-c74dcde1f472</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;data\&quot; : [\n \t{\n\t\&quot;title\&quot; : \&quot;${var_title}\&quot;,\n    \&quot;body\&quot; : \&quot;${var_body}\&quot;,\n    \&quot;userId\&quot; : 1\n\t},\n\t{\n\t\&quot;title\&quot; : \&quot;test\&quot;,\n    \&quot;body\&quot; : \&quot;test\&quot;,\n    \&quot;userId\&quot; : 1\n\t}\n  ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e80fe49c-eb1c-4705-818b-80a7a7b4872a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ini title update'</defaultValue>
      <description></description>
      <id>9260879b-2645-4457-ac52-0cf73bd5a3bc</id>
      <masked>false</masked>
      <name>var_title</name>
   </variables>
   <variables>
      <defaultValue>'ini body update'</defaultValue>
      <description></description>
      <id>3d3b20ce-d046-4f40-a8ec-3e360e397bf3</id>
      <masked>false</masked>
      <name>var_body</name>
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

//verifikasi status code 201

WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)

////verifikasi data yang telah di create
//WS.verifyElementPropertyValue(response, 'title', 'ini title update')
//WS.verifyElementPropertyValue(response, 'body', 'ini body update')
//WS.verifyElementPropertyValue(response, 'userId', '1')
//WS.verifyElementPropertyValue(response, 'id', '101')


//verfikasi data yang telah di create
WS.verifyElementPropertyValue(response, 'data[0].title', 'ini title update')
WS.verifyElementPropertyValue(response, 'data[0].body', 'ini body update')
WS.verifyElementPropertyValue(response, 'data[0].userId', '1')
WS.verifyElementPropertyValue(response, 'data[1].title', 'test')
WS.verifyElementPropertyValue(response, 'data[1].body', 'test')
WS.verifyElementPropertyValue(response, 'data[1].userId', '1')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
