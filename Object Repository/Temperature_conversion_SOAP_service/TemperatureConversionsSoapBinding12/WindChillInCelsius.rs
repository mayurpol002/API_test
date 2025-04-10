<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>WindChillInCelsius</name>
   <tag></tag>
   <elementGuidId>6cae3eaf-8078-40a1-8d50-0498503bf503</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>dc8b7410-560b-4c39-865c-bbb52d115e87</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <path></path>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://www.w3.org/2003/05/soap-envelope&quot; xmlns:tem=&quot;http://webservices.daehosting.com/temperature&quot;>&#xd;
   &lt;soap:Header/>&#xd;
   &lt;soap:Body>&#xd;
      &lt;tem:WindChillInCelsius>&#xd;
         &lt;tem:nCelsius>1000.00&lt;/tem:nCelsius>&#xd;
         &lt;tem:nWindSpeed>1000.00&lt;/tem:nWindSpeed>&#xd;
      &lt;/tem:WindChillInCelsius>&#xd;
   &lt;/soap:Body>&#xd;
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceEndpoint>http://webservices.daehosting.com/services/TemperatureConversions.wso</soapServiceEndpoint>
   <soapServiceFunction>WindChillInCelsius</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://webservices.daehosting.com/services/TemperatureConversions.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
